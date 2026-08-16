//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 840/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk840(t1337: f64, t180: f64, t479: f64, t306: f64, t425: f64, t442: f64, t1056: f64, t1175: f64, t1364: f64, t3521: f64, t3541: f64, t3546: f64) -> (f64, f64, f64, f64) {
    let t12847 = t180 * t479 * t1337;
    let t12848 = t306 * t425;
    let t12849 = t12848 * t442;
    let t12850 = t1056 * t1175;
    let t12852 = t12849 * t12850 * t1364;
    let t12855 = t3521 * t3541;
    let t12857 = t3521 * t3546;
    (t12847, t12852, t12855, t12857)
}
