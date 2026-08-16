//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1377/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1377(t29821: f64, t29848: f64, t29882: f64, t29911: f64, t21393: f64, t21396: f64, t21427: f64, t21430: f64, t21433: f64, t21638: f64, t21641: f64, t25214: f64, t25217: f64, t25220: f64, t29819: f64) -> (f64, f64) {
    let t29913 = t29821 + t29848 + t29882 + t29911;
    let t29932 = t21638 - 0.32136222222222222222e1_f64 * t21393 + 0.68863333333333333333e0_f64 * t21396 + t21641 + 0.34731666666666666666e0_f64 * t21430 - 0.18523555555555555555e1_f64 * t21427 + 0.34731666666666666666e0_f64 * t21433 - 0.32136222222222222223e1_f64 * t25214 + 0.27545333333333333334e1_f64 * t25217 - 0.103295e1_f64 * t25220 + 0.3529725e1_f64 * t29819;
    (t29913, t29932)
}
