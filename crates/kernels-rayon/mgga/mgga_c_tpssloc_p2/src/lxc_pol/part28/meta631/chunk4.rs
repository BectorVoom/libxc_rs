//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1981/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1981(t87328: f64, t87330: f64, t87332: f64, t87338: f64, t87341: f64, t87345: f64, t87347: f64, t87363: f64, t87335: f64, t87343: f64, t87351: f64, t87355: f64, t87359: f64, t87365: f64, t87369: f64, t87371: f64, t87373: f64, t87375: f64) -> f64 {
    let t92645 = 0.80745512188280781706e-3_f64 * t87328;
    let t92646 = 7.0_f64 / 144.0_f64 * t87330;
    let t92647 = 7.0_f64 / 144.0_f64 * t87332;
    let t92649 = 0.13457585364713463618e-3_f64 * t87338;
    let t92650 = 7.0_f64 / 144.0_f64 * t87341;
    let t92652 = 119.0_f64 / 864.0_f64 * t87345;
    let t92653 = 0.11304371706359309439e-1_f64 * t87347;
    let t92657 = 7.0_f64 / 288.0_f64 * t87363;
    let t92663 = -t92645 + t92646 + t92647 - 0.80745512188280781706e-3_f64 * t87335 + t92649 + t92650 - t87343 / 192.0_f64 - t92652 - t92653 - 0.16956557559538964158e-1_f64 * t87351 - 0.24223653656484234512e-2_f64 * t87355 - 0.24223653656484234512e-2_f64 * t87359 - t92657 - 5.0_f64 / 192.0_f64 * t87365 - t87369 / 128.0_f64 + t87371 / 128.0_f64 - t87373 / 768.0_f64 - t87375 / 96.0_f64;
    t92663
}
