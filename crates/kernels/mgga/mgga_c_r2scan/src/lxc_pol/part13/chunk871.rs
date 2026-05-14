//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 871/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk871<F: Float>(t10868: F, t2150: F, t2147: F, t545: F, t775: F) -> (F, F, F, F) {
    let t10869 = t10868 * t2150;
    let t10870 = t2147 * t10869;
    let t10871 = 0.46574606203128791246e-1 * t10870;
    let t10872 = t545 * t775;
    (t10869, t10870, t10871, t10872)
}
