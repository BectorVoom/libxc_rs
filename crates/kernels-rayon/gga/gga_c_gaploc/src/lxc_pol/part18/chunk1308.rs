//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1308/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1308(t32357: f64, t4820: f64, t7513: f64, t32436: f64, t10678: f64, t11095: f64, t1402: f64, t1628: f64, t2033: f64, t33453: f64, t33455: f64, t33458: f64, t33460: f64, t33462: f64, t33465: f64, t33469: f64, t33474: f64, t33476: f64, t33478: f64, t33480: f64, t3495: f64, t4598: f64, t813: f64) -> f64 {
    let t33483 = 0.15889106645266856297e0_f64 * t7513 * t4820 * t32357;
    let t33486 = 0.15889106645266856297e0_f64 * t7513 * t4820 * t32436;
    let t33487 = -0.61348681526273199482e1_f64 * t813 * t1628 * t11095 - 0.1022478025437886658e1_f64 * t813 * t4598 * t3495 - t33453 + t33455 - t33458 + t33460 - t33462 - t33465 + t33469 - 0.92686455430723328401e-1_f64 * t2033 * t1402 * t10678 - t33474 + t33476 + t33478 - t33480 - t33483 - t33486;
    t33487
}
