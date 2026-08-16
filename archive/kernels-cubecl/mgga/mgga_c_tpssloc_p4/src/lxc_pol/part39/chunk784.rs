//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 784/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk784<F: Float>(t1484: F, t776: F, t2701: F, t820: F, t4119: F, t847: F, t1516: F, t2621: F, t2623: F, t2640: F, t2643: F, t2695: F, t2698: F, t4191: F, t4236: F, t4240: F, t4250: F, t4253: F, t817: F, t843: F) -> (F, F, F, F) {
    let t4255 = t1484 * t776;
    let t4257 = t2701 * t820 * t4255;
    let t4261 = t847 * t820 * t4119;
    let t4264 = t2643 * t4191 / F::cast_from(768.0_f64) - t817 * t4236 / F::cast_from(3072.0_f64) - t2643 * t4240 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t2621 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t2640 + t2695 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t2698 - t2623 * t1516 / F::cast_from(768.0_f64) + t2643 * t4250 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t4253 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t843 * t4257 - t843 * t4261 / F::cast_from(768.0_f64);
    (t4255, t4257, t4261, t4264)
}
