//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1714/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1714(t13652: f64, t177: f64, t6800: f64, t762: f64, t13666: f64, t13668: f64, t9858: f64, t9861: f64, t13887: f64, t13664: f64, t13682: f64, t13683: f64, t9524: f64, t9542: f64, t9588: f64, t9854: f64, t9865: f64, t9868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22211 = 0.34631718211362927517e2_f64 * t13652;
    let t22212 = t6800 * t177;
    let t22213 = t22212 * t762;
    let t22214 = 0.5848223622634646207e0_f64 * t22213;
    let t22215 = 0.21687162600603479684e-1_f64 * t13666;
    let t22216 = 24.0_f64 * t13668;
    let t22217 = 0.17315859105681463759e2_f64 * t9858;
    let t22218 = 0.10843581300301739842e-1_f64 * t9861;
    let t22219 = 0.48830526149350786811e-3_f64 * t13887;
    let t22220 = -t22211 - t9588 - t9524 - t13664 - t22214 + t22215 - t22216 + t9542 + t13682 + t9854 + t13683 - t22217 + t22218 + t9865 + t9868 + t22219;
    (t22211, t22214, t22215, t22216, t22217, t22218, t22219, t22220)
}
