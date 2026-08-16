//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1855/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1855(t91154: f64, t91158: f64, t91161: f64, t91170: f64, t91214: f64, t91225: f64, t91281: f64, t91283: f64, t91286: f64, t91290: f64, t91300: f64, t91303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93651 = 0.13457585364713463618e-3_f64 * t91154;
    let t93652 = 0.26915170729426927236e-3_f64 * t91158;
    let t93653 = 7.0_f64 / 144.0_f64 * t91161;
    let t93657 = 7.0_f64 / 12.0_f64 * t91170;
    let t93674 = 7.0_f64 / 144.0_f64 * t91214;
    let t93682 = 0.56521858531796547194e-2_f64 * t91225;
    let t93710 = 7.0_f64 / 576.0_f64 * t91281;
    let t93711 = 7.0_f64 / 576.0_f64 * t91283;
    let t93712 = 7.0_f64 / 576.0_f64 * t91286;
    let t93715 = 0.33913115119077928316e-1_f64 * t91290;
    let t93718 = 0.11304371706359309439e-1_f64 * t91300;
    let t93720 = 7.0_f64 / 576.0_f64 * t91303;
    (t93651, t93652, t93653, t93657, t93674, t93682, t93710, t93711, t93712, t93715, t93718, t93720)
}
