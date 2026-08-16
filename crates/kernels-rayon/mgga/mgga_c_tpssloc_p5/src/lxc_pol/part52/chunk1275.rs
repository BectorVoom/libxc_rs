//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1275/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1275(t4162: f64, t8342: f64, t8344: f64, t23083: f64, t32837: f64, t23062: f64, t32834: f64, t112778: f64, t112784: f64, t112803: f64, t118533: f64, t118535: f64, t118539: f64, t118546: f64, t118549: f64, t118552: f64, t118556: f64, t118559: f64, t118562: f64, t118566: f64, t118569: f64, t118573: f64) -> f64 {
    let t118576 = t4162 * t8342 * t8344;
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118582 = -t118533 / 1536.0_f64 - t118535 / 1536.0_f64 - t118539 / 1536.0_f64 + 5.0_f64 / 384.0_f64 * t118546 - 0.80745512188280781708e-3_f64 * t118549 + 0.33913115119077928318e-1_f64 * t118552 + 0.13457585364713463618e-3_f64 * t112778 + 0.16149102437656156342e-2_f64 * t118556 + 0.48447307312968469025e-2_f64 * t118559 + 0.33913115119077928318e-1_f64 * t112784 + t118562 / 768.0_f64 + 7.0_f64 / 2304.0_f64 * t112803 + 0.48447307312968469025e-2_f64 * t118566 - 0.80745512188280781708e-3_f64 * t118569 + 0.80745512188280781708e-3_f64 * t118573 + t118576 / 1536.0_f64 + 0.56521858531796547196e-2_f64 * t118578 + 0.33913115119077928318e-1_f64 * t118580;
    t118582
}
