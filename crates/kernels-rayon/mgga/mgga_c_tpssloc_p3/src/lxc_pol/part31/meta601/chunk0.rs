//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1846/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1846(t87247: f64, t87255: f64, t87262: f64, t87270: f64, t87272: f64, t87291: f64, t87293: f64, t87300: f64, t87308: f64, t87328: f64, t87330: f64, t87332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92599 = 7.0_f64 / 576.0_f64 * t87247;
    let t92603 = 7.0_f64 / 576.0_f64 * t87255;
    let t92607 = 7.0_f64 / 576.0_f64 * t87262;
    let t92614 = 7.0_f64 / 144.0_f64 * t87270;
    let t92615 = 7.0_f64 / 576.0_f64 * t87272;
    let t92626 = 7.0_f64 / 36.0_f64 * t87291;
    let t92627 = 0.33913115119077928316e-1_f64 * t87293;
    let t92630 = 35.0_f64 / 144.0_f64 * t87300;
    let t92635 = 0.33913115119077928316e-1_f64 * t87308;
    let t92645 = 0.80745512188280781706e-3_f64 * t87328;
    let t92646 = 7.0_f64 / 144.0_f64 * t87330;
    let t92647 = 7.0_f64 / 144.0_f64 * t87332;
    (t92599, t92603, t92607, t92614, t92615, t92626, t92627, t92630, t92635, t92645, t92646, t92647)
}
