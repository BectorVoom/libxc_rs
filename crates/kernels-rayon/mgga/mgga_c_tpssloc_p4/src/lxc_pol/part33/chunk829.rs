//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 829/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk829(t10475: f64, t10478: f64, t10472: f64, t3131: f64, t360: f64, t376: f64, t676: f64, t2928: f64, t320: f64, t10294: f64, t268: f64, t271: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10508 = t676 * t376;
    let t10523 = 1.0_f64 / t2928 / t320;
    let t10542 = 0.36793333333333333333e0_f64 * t10294;
    let t10544 = t268 * t6546 * t271;
    (t10480, t10482, t10508, t10523, t10542, t10544)
}
