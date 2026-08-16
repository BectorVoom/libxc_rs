//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 960/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk960(t3272: f64, t774: f64, t1232: f64, t1639: f64, t3260: f64, t3342: f64, t4480: f64, t10077: f64, t1642: f64, t10160: f64, t1630: f64, t125: f64, t4459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12822 = t3272 * t774;
    let t12823 = t1639 * t1232;
    let t12828 = t1639 * t3260;
    let t12835 = 35.0_f64 / 576.0_f64 * t3342 * t4480;
    let t12846 = t10077 * t1642;
    let t12861 = t10160 * t1630;
    let t12863 = t125 * t4459;
    (t12822, t12823, t12828, t12835, t12846, t12861, t12863)
}
