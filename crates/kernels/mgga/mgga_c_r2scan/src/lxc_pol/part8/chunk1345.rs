//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1345/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1345<F: Float>(t19037: F, t19041: F, t19048: F, t19057: F, t19061: F, t19069: F, t19341: F, t19388: F, t23781: F, t23798: F, t32208: F, t32209: F, t32210: F, t32217: F, t32218: F, t32219: F, t32228: F) -> (F,) {
    let t32960 = t19037 + t32208 - t19041 - t19048 + t32209 - t32210 - t19057 + t19061 - t32217 + t32218 + t19069 - t23781 - t19341 - t32219 + t23798 - t32228 - t19388;
    (t32960,)
}
