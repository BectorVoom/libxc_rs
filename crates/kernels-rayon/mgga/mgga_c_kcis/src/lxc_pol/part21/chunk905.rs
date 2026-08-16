//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 905/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk905(t13712: f64, t10218: f64, t13710: f64, t13714: f64, t13723: f64, t13732: f64, t13767: f64, t13772: f64, t13775: f64, t13777: f64, t9700: f64, t13717: f64, t13720: f64, t13726: f64, t13729: f64, t13735: f64, t13738: f64, t13742: f64, t9681: f64, t9683: f64, t9691: f64) -> f64 {
    let t13781 = 0.18344444444444444444e-2_f64 * t13712;
    let t13782 = -0.27516666666666666666e-2_f64 * t9700 + 0.1982e-1_f64 * t13767 + 0.1651e-1_f64 * t13723 - 0.24765e-1_f64 * t13732 + 0.14865e-1_f64 * t13772 - t10218 - 0.1982e-1_f64 * t13775 - 0.991e-2_f64 * t13777 - 0.18344444444444444444e-2_f64 * t13710 - 0.55033333333333333333e-2_f64 * t13714 + t13781;
    let t13783 = -0.27516666666666666667e-2_f64 * t13729 - 0.45861111111111111112e-2_f64 * t13720 - 0.11006666666666666667e-1_f64 * t13726 + 0.8255e-2_f64 * t13738 + 0.3302e-1_f64 * t13735 + 0.30268333333333333334e-1_f64 * t13717 - 0.8255e-2_f64 * t13742 + 0.13758333333333333333e-2_f64 * t9681 + 0.9172222222222222222e-3_f64 * t9683 - 0.36688888888888888888e-2_f64 * t9691 + t13782;
    t13783
}
