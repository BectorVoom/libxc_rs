//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1599/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1599<F: Float>(t1487: F, t1494: F, t21686: F, t21784: F, t21794: F, t22662: F, t22665: F, t22671: F, t22719: F, t22739: F, t2299: F, t2306: F, t38: F, t4227: F, t4232: F, t46001: F, t46014: F, t5819: F, t5820: F, t5825: F, t5854: F, t5855: F, t5869: F, t633: F, t637: F, t70: F, t71: F, t77: F, t7719: F, t85: F, t85161: F, t87107: F, t87126: F, t87145: F, t87155: F) -> F {
    let t87195 = -t5819 * t5854 * t85 / F::cast_from(2.0_f64) - t22665 * t1494 - t5820 * t5869 / F::cast_from(2.0_f64) + t38 * t87155 * t85 / F::cast_from(24.0_f64) + t22719 * t1494 / F::cast_from(6.0_f64) + t5855 * t5869 / F::cast_from(4.0_f64) + t1487 * t22739 / F::cast_from(6.0_f64) + t71 * t77 * (F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46001 * t87145 - F::cast_from(560.0_f64) / F::cast_from(9.0_f64) * t21784 * t5825 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2299 * t87107 + F::cast_from(112.0_f64) / F::cast_from(9.0_f64) * t4227 * t22671 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t87126 + F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46014 * t87145 + F::cast_from(560.0_f64) / F::cast_from(9.0_f64) * t21794 * t5825 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2306 * t87107 + F::cast_from(112.0_f64) / F::cast_from(9.0_f64) * t4232 * t22671 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t87126) / F::cast_from(24.0_f64) - t87107 * t70 * t85 / F::cast_from(4.0_f64) - t85161 * t22662 - t21686 * t7719 * t5825;
    t87195
}
