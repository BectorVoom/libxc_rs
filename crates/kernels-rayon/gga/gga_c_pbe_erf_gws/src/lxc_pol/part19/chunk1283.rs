//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1283/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1283(t15377: f64, t2397: f64, t15182: f64, t51666: f64, t14733: f64, t8690: f64, t11407: f64, t14797: f64, t3989: f64, t3990: f64, t12237: f64, t13780: f64, t14637: f64) -> (f64, f64, f64, f64, f64) {
    let t56351 = t15377 * t2397;
    let t56357 = t51666 * t15182;
    let t56362 = t14733 * t8690;
    let t56366 = t3989 * t3990 * t14797 * t11407;
    let t56374 = t14637 * t3990 * t13780 * t12237;
    (t56351, t56357, t56362, t56366, t56374)
}
