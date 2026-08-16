//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1856/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1856(t91310: f64, t91327: f64, t91344: f64, t91356: f64, t91358: f64, t91364: f64, t91386: f64, t91402: f64, t91404: f64, t91064: f64, t91076: f64, t90723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93722 = 0.13457585364713463618e-3_f64 * t91310;
    let t93731 = 0.80745512188280781706e-3_f64 * t91327;
    let t93736 = 0.56521858531796547194e-2_f64 * t91344;
    let t93742 = 0.33913115119077928316e-1_f64 * t91356;
    let t93743 = 0.56521858531796547194e-2_f64 * t91358;
    let t93745 = 7.0_f64 / 144.0_f64 * t91364;
    let t93753 = 35.0_f64 / 144.0_f64 * t91386;
    let t93762 = 7.0_f64 / 36.0_f64 * t91402;
    let t93763 = 0.33913115119077928316e-1_f64 * t91404;
    let t93792 = 0.15352717957250113407e0_f64 * t91064;
    let t93794 = 0.76763589786250567036e-1_f64 * t91076;
    let t93824 = 0.16449340668482264365e-1_f64 * t90723;
    (t93722, t93731, t93736, t93742, t93743, t93745, t93753, t93762, t93763, t93792, t93794, t93824)
}
