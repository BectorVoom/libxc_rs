//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 722/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk722(t140: f64, t3529: f64, t3737: f64, t11250: f64, t461: f64, t1337: f64, t180: f64, t479: f64, t306: f64, t425: f64, t442: f64, t3831: f64, t458: f64) -> (f64, f64, f64, f64, f64) {
    let t12841 = t140 * t3737 * t3529;
    let t12845 = 0.29201909629629629629e-3_f64 * t11250 * t461;
    let t12847 = t180 * t479 * t1337;
    let t12848 = t306 * t425;
    let t12849 = t12848 * t442;
    let t12872 = t458 * t3831;
    (t12841, t12845, t12847, t12849, t12872)
}
