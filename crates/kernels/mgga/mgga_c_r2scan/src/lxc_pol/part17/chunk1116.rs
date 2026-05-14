//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1116/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1116<F: Float>(t45046: F, t45048: F, t45053: F, t45058: F, t45060: F, t45066: F, t45068: F, t45070: F, t45073: F, t45078: F, t45083: F, t45085: F, t45088: F, t45094: F, t45097: F, t45099: F, t45100: F) -> (F,) {
    let t45110 = -t45046 - t45048 + t45053 - t45058 + t45060 + t45066 + t45068 - t45070 + t45073 + t45078 + t45083 + t45085 + t45088 - t45094 - t45097 + t45099 + t45100;
    (t45110,)
}
