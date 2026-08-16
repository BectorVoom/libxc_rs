//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1847/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1847(t87338: f64, t87341: f64, t87347: f64, t87363: f64, t87401: f64, t87411: f64, t87443: f64, t87463: f64, t87477: f64, t87487: f64, t87565: f64, t87581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92649 = 0.13457585364713463618e-3_f64 * t87338;
    let t92650 = 7.0_f64 / 144.0_f64 * t87341;
    let t92653 = 0.11304371706359309439e-1_f64 * t87347;
    let t92657 = 7.0_f64 / 288.0_f64 * t87363;
    let t92675 = 7.0_f64 / 576.0_f64 * t87401;
    let t92679 = 0.56521858531796547194e-2_f64 * t87411;
    let t92697 = 0.80745512188280781706e-3_f64 * t87443;
    let t92705 = 7.0_f64 / 12.0_f64 * t87463;
    let t92710 = 0.33913115119077928316e-1_f64 * t87477;
    let t92713 = 0.56521858531796547194e-2_f64 * t87487;
    let t92729 = 0.15352717957250113407e0_f64 * t87565;
    let t92738 = 0.16449340668482264365e-1_f64 * t87581;
    (t92649, t92650, t92653, t92657, t92675, t92679, t92697, t92705, t92710, t92713, t92729, t92738)
}
