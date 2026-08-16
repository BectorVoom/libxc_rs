//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2381/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2381(t20217: f64, t2775: f64, t607: f64, t136: f64, t908: f64, t2770: f64, t2826: f64, t21118: f64, t3966: f64, t5677: f64, t68481: f64, t13541: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t68534 = t2775 * t20217 * t607;
    let t68536 = t136 * t908 * t68534;
    let t68539 = t2770 * t20217 * t607;
    let t68541 = t136 * t2826 * t68539;
    let t68543 = t21118 * t607;
    let t68545 = t136 * t908 * t68543;
    let t68547 = t5677 * t3966;
    let t68549 = t136 * t908 * t68547;
    let t68552 = t136 * t2826 * t68481;
    let t68554 = t13541 * t5398;
    (t68534, t68536, t68539, t68541, t68543, t68545, t68547, t68549, t68552, t68554)
}
