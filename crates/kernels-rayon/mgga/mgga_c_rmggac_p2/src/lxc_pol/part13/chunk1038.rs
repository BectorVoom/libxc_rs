//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1038/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1038(t38559: f64, t38562: f64, t38539: f64, t38541: f64, t38545: f64, t38550: f64, t38552: f64, t38554: f64, t38556: f64, t38566: f64, t38570: f64, t38572: f64, t38574: f64, t38576: f64, t38578: f64, t38583: f64, t38588: f64) -> f64 {
    let t42665 = 0.162600798888400151e-2_f64 * t38559;
    let t42666 = 0.162600798888400151e-2_f64 * t38562;
    let t42675 = -0.638468998399467591e-4_f64 * t38539 + 0.1702583995731913576e-4_f64 * t38541 + 0.1702583995731913576e-4_f64 * t38545 + 0.638468998399467591e-4_f64 * t38550 + 0.60975299583150056624e-3_f64 * t38552 + 0.60975299583150056624e-3_f64 * t38554 - 0.7044137609176975208e-2_f64 * t38556 - t42665 - t42666 + 0.40911992481368012596e0_f64 * t38566 - 0.14546486215597515589e0_f64 * t38570 + 0.10215503974391481456e-3_f64 * t38572 - 0.15323255961587222184e-3_f64 * t38574 - 0.5107751987195740728e-4_f64 * t38576 + 0.5107751987195740728e-4_f64 * t38578 - 0.638468998399467591e-4_f64 * t38583 + 0.3405167991463827152e-4_f64 * t38588;
    t42675
}
