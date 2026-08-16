//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 914/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk914(t52: f64, t2440: f64, t5392: f64, t5398: f64, t76: f64, t5512: f64, t145: f64, t185: f64, t157: f64, t182: f64, t4200: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2522: f64, t5497: f64, t5498: f64, t5501: f64, t5502: f64, t5506: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t150 = t52 <= zeta_threshold;
    let t5518 = piecewise3(t150, 0.0_f64, 4.0_f64 / 9.0_f64 * t2440 * t5392 - 4.0_f64 / 3.0_f64 * t76 * t5398);
    let t5519 = t5512 + t5518;
    let t5520 = t145 * t5519;
    let t5521 = t5520 * t185;
    let t5522 = t5519 * t157;
    let t5524 = 0.19751673498613801407e-1_f64 * t5522 * t182;
    let t5525 = 0.11696447245269292414e1_f64 * t4200;
    let t5526 = 6.0_f64 * t2522 * t5502 + t2373 + t2377 + t2408 + t2417 + t5497 + t5498 + t5501 + t5506 + t5521 + t5524 - t5525;
    (t5519, t5520, t5521, t5522, t5524, t5525, t5526)
}
