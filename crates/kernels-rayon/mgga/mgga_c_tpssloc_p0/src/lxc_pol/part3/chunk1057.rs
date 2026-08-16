//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1057/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1057(t10599: f64, t1547: f64, t2799: f64, t13615: f64, t894: f64, t1553: f64, t2403: f64, t4392: f64, t699: f64, t13611: f64, t908: f64, t136: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13637 = t10599 * t1547;
    let t13638 = t13637 * t2799;
    let t13640 = t894 * t13615;
    let t13642 = t2403 * t1553;
    let t13644 = t699 * t4392;
    let t13645 = 0.10954222222222222222e0_f64 * t13644;
    let t13646 = t908 * t13611;
    let t13647 = t136 * t13646;
    (t13638, t13640, t13642, t13644, t13645, t13647)
}
