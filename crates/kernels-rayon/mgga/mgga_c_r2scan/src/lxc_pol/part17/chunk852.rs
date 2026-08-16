//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 852/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk852(t5834: f64, t5966: f64, t5968: f64, t5970: f64, t5972: f64, t5975: f64, t5976: f64, t5978: f64, t5980: f64, t5982: f64, t5985: f64, t7849: f64) -> f64 {
    let t9025 = -t5966 + 0.21687162600603479684e-1_f64 * t5968 - 0.32106488758451047386e0_f64 * t5970 - 0.1301229756036208781e0_f64 * t5972 - t5975 + 8.0_f64 * t5976 - 0.11290853155555555555e-2_f64 * t5978 + t5834 + 8.0_f64 * t5980 - 20.0_f64 * t5982 + t5985 + t7849;
    t9025
}
