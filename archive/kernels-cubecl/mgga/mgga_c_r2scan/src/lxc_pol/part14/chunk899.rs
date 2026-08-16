//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 899/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk899<F: Float>(t7308: F, t7363: F, t7412: F, t7446: F, t7488: F, t7531: F, t7588: F, t7929: F, t7982: F, t8042: F, t8101: F, t8142: F, t8184: F, t8225: F, t8255: F, t8292: F) -> F {
    let t8296 = t7308 + t7363 + t7412 + t7446 + t7488 + t7531 + t7588 + t7929 + t7982 + t8042 + t8101 + t8142 + t8184 + t8225 + t8255 + t8292;
    t8296
}
