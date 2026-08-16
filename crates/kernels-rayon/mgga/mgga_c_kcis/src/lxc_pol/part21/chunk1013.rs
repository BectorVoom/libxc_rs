//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1013/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1013(t1823: f64, t3574: f64, t13908: f64, t13720: f64, t13726: f64, t13729: f64, t13735: f64, t13738: f64, t9700: f64, t9702: f64, t9708: f64, t9710: f64, t9712: f64) -> (f64, f64) {
    let t15369 = t1823 * t3574;
    let t15397 = 0.27785333333333333334e0_f64 * t13908;
    let t15398 = -0.34431666666666666666e0_f64 * t9700 - 0.13892666666666666667e0_f64 * t9702 - 0.23154444444444444444e0_f64 * t9708 + 0.69463333333333333333e-1_f64 * t9710 + 0.23154444444444444444e-1_f64 * t9712 - 0.34431666666666666667e0_f64 * t13729 - 0.57386111111111111112e0_f64 * t13720 - 0.13772666666666666667e1_f64 * t13726 + 0.103295e1_f64 * t13738 + 0.41318e1_f64 * t13735 - t15397;
    (t15369, t15398)
}
