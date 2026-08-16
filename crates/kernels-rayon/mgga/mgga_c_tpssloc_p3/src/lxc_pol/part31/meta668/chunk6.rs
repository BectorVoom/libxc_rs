//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1971/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1971(t81903: f64, t87335: f64, t87345: f64, t87387: f64, t92646: f64, t92647: f64, t92649: f64, t92650: f64, t92653: f64, t92657: f64, t92675: f64, t98796: f64, t98798: f64, t98801: f64, t98803: f64, t98808: f64, t98811: f64, t98814: f64) -> f64 {
    let t101468 = t92646 + t92647 - 0.80745512188280781708e-3_f64 * t87335 + t92649 + t92650 - 119.0_f64 / 432.0_f64 * t87345 - 7.0_f64 / 576.0_f64 * t98796 + 7.0_f64 / 1152.0_f64 * t98798 - t92653 - 0.40372756094140390853e-3_f64 * t98801 - t98803 / 48.0_f64 - t92657 + 0.20186378047070195426e-3_f64 * t81903 - 0.126501302428306558e-1_f64 * t87387 - t98808 / 2.0_f64 + t98811 / 4.0_f64 - 0.13565246047631171326e0_f64 * t98814 - t92675;
    t101468
}
