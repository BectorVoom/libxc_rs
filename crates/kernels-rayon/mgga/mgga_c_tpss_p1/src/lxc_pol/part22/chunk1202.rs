//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1202/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1202(t18454: f64, t3277: f64, t3329: f64, t5721: f64, t3334: f64, t1765: f64, t3338: f64, t339: f64, t5726: f64, t789: f64) -> (f64, f64, f64, f64, f64) {
    let t18455 = t18454 * t3277;
    let t18457 = t5721 * t3329;
    let t18459 = t5721 * t3334;
    let t18461 = t1765 * t3338;
    let t18464 = t339 * t5726 * t789;
    (t18455, t18457, t18459, t18461, t18464)
}
