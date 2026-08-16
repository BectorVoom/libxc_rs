//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3805/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3805<F: Float>(t21639: F, t3794: F, t5023: F, t68735: F, t68738: F, t68742: F, t68744: F, t68746: F, t68748: F, t68751: F, t68754: F, t68757: F, t68760: F, t68763: F, t68766: F, t68769: F) -> F {
    let t73270 = F::cast_from(2.0_f64) * t21639 * t3794 * t5023 - t68735 + t68738 + t68742 + t68744 - t68746 - t68748 + t68751 + t68754 - t68757 - t68760 - t68763 - t68766 - t68769;
    t73270
}
