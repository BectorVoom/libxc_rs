//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1115/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1115<F: Float>(t1471: F, t1487: F, t1494: F, t21686: F, t22662: F, t22665: F, t22673: F, t22676: F, t22681: F, t22719: F, t22739: F, t5820: F, t5827: F, t5830: F, t5855: F, t5869: F, t71: F, t85: F) -> F {
    let t22742 = -t21686 * t22662 / F::new(4.0) - t22665 * t85 / F::new(4.0) - t5820 * t1494 / F::new(4.0) - t22673 * t85 / F::new(12.0) - t22676 * t85 / F::new(4.0) - t5827 * t1494 / F::new(4.0) - t22681 * t85 / F::new(4.0) - t5830 * t1494 / F::new(2.0) - t1471 * t5869 / F::new(4.0) + t22719 * t85 / F::new(24.0) + t5855 * t1494 / F::new(8.0) + t1487 * t5869 / F::new(8.0) + t71 * t22739 / F::new(24.0);
    t22742
}
