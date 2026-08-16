//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2382/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2382(t136: f64, t68554: f64, t908: f64, t43317: f64, t48140: f64, t68513: f64, t49200: f64, t59657: f64, t60163: f64, t60168: f64, t60173: f64, t68536: f64, t68541: f64, t68545: f64, t68549: f64, t68552: f64) -> (f64, f64, f64) {
    let t68556 = t136 * t908 * t68554;
    let t68563 = t48140 * t43317 * t68513;
    let t68565 = 0.16557e0_f64 * t68536 - 0.27595e-1_f64 * t68541 + 0.198684e1_f64 * t68545 - 0.149013e1_f64 * t68549 - 0.99342e0_f64 * t68552 + 0.49671e0_f64 * t68556 + 0.16557e0_f64 * t60163 + 0.5519e0_f64 * t60168 - 0.27595e0_f64 * t60173 - 0.26837777777777777777e0_f64 * t59657 - 0.11038e0_f64 * t68563 + t49200;
    (t68556, t68563, t68565)
}
