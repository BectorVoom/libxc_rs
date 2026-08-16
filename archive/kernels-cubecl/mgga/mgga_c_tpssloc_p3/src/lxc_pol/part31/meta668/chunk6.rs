//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1971/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1971<F: Float>(t81903: F, t87335: F, t87345: F, t87387: F, t92646: F, t92647: F, t92649: F, t92650: F, t92653: F, t92657: F, t92675: F, t98796: F, t98798: F, t98801: F, t98803: F, t98808: F, t98811: F, t98814: F) -> F {
    let t101468 = t92646 + t92647 - F::cast_from(0.80745512188280781708e-3_f64) * t87335 + t92649 + t92650 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t87345 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t98796 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t98798 - t92653 - F::cast_from(0.40372756094140390853e-3_f64) * t98801 - t98803 / F::cast_from(48.0_f64) - t92657 + F::cast_from(0.20186378047070195426e-3_f64) * t81903 - F::cast_from(0.126501302428306558e-1_f64) * t87387 - t98808 / F::cast_from(2.0_f64) + t98811 / F::cast_from(4.0_f64) - F::cast_from(0.13565246047631171326e0_f64) * t98814 - t92675;
    t101468
}
