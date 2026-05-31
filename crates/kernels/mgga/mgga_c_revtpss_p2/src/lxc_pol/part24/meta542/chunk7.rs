//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1600/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1600<F: Float>(t1470: F, t1471: F, t1486: F, t1494: F, t1927: F, t21686: F, t22671: F, t22672: F, t22673: F, t22676: F, t22681: F, t22718: F, t22739: F, t36: F, t5826: F, t5827: F, t5830: F, t5854: F, t5869: F, t70: F, t85: F, t87126: F) -> F {
    let t87221 = -t21686 * t1927 * t22671 / F::cast_from(3.0_f64) - t36 * t87126 * t70 * t85 / F::cast_from(12.0_f64) - t22672 * t1486 * t85 / F::cast_from(3.0_f64) - t22673 * t1494 / F::cast_from(3.0_f64) - t5826 * t5854 * t85 / F::cast_from(2.0_f64) - t22676 * t1494 - t5827 * t5869 / F::cast_from(2.0_f64) - t1470 * t22718 * t85 / F::cast_from(3.0_f64) - t22681 * t1494 - t5830 * t5869 - t1471 * t22739 / F::cast_from(3.0_f64);
    t87221
}
