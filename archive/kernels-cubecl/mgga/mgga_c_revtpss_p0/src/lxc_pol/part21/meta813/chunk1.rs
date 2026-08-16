//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2978/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2978<F: Float>(t52510: F, t52516: F, t52899: F, t52905: F, t52910: F, t52912: F, t52914: F, t52916: F, t52918: F, t52920: F, t54230: F, t54231: F, t54233: F, t54246: F) -> F {
    let t54249 = t54230 + t54231 + t54233 - t52510 + t52899 - t52905 - t52910 - t52516 - t52912 + t52914 - t52916 - t52918 - t52920 + t54246;
    t54249
}
