//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3636/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3636(t20887: f64, t3531: f64, t1196: f64, t20886: f64, t3516: f64, t43771: f64, t43781: f64, t43783: f64, t44039: f64, t44040: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64) -> (f64, f64, f64) {
    let t68805 = 0.23392894490538584828e1_f64 * t3531 * t20887;
    let t68808 = 0.11696447245269292414e1_f64 * t1196 * t20886 * t3516;
    let t68821 = 0.11958666666666666667e1_f64 * t68253 + 0.13287407407407407408e0_f64 * t68255 - 0.88582716049382716049e-1_f64 * t68257 - 0.486854320987654321e0_f64 * t43771 + 0.91285185185185185187e-1_f64 * t43781 + 0.18257037037037037037e0_f64 * t43783 + t44039 + t44040 - 0.22145679012345679012e0_f64 * t68262 + 0.33218518518518518518e0_f64 * t68267 + 0.71752e1_f64 * t68271 + 0.11958666666666666667e1_f64 * t68275 - 0.39862222222222222222e0_f64 * t68277 - 0.39862222222222222222e0_f64 * t68282;
    (t68805, t68808, t68821)
}
