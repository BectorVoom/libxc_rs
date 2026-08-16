//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2888/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2888(t136: f64, t59696: f64, t908: f64, t2826: f64, t59742: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64, t59727: f64, t59732: f64, t59735: f64, t59738: f64, t59744: f64) -> (f64, f64, f64) {
    let t60282 = t136 * t908 * t59696;
    let t60296 = t136 * t2826 * t59742;
    let t60300 = 0.16557e0_f64 * t60282 - 0.80513333333333333333e0_f64 * t59700 + 0.26837777777777777778e0_f64 * t59702 + 0.22364814814814814814e0_f64 * t59704 - 0.33547222222222222222e0_f64 * t59708 - 0.89459259259259259259e0_f64 * t59713 + 0.12077e1_f64 * t59717 - 0.40256666666666666666e0_f64 * t59721 - 0.33547222222222222222e0_f64 * t59727 + 0.12077e1_f64 * t59732 - 0.13418888888888888889e1_f64 * t59735 + 0.48307999999999999999e1_f64 * t59738 + 0.16557e0_f64 * t60296 + 0.62621481481481481484e0_f64 * t47787 + 0.12077e1_f64 * t59744;
    (t60282, t60296, t60300)
}
