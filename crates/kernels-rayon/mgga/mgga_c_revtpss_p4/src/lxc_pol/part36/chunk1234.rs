//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1234/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1234(t94973: f64, t239: f64, t655: f64, t2339: f64, t624: f64, t10208: f64, t68: f64, t10368: f64, t55: f64, t45972: f64, t7565: f64, t12627: f64, t2142: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94974 = 154.0_f64 / 27.0_f64 * t94973;
    let t94975 = t239 * t655;
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t96733 = t55 * t10368;
    let t96804 = t45972 * t7565;
    let t96861 = t12627 * t2142;
    (t94974, t94975, t94978, t94982, t96733, t96804, t96861)
}
