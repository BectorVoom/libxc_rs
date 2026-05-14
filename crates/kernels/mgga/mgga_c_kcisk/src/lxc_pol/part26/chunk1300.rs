//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1300/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1300<F: Float>(t109152: F, t109154: F, t109160: F, t109162: F, t109165: F, t110815: F, t110817: F, t111524: F, t1155: F, t2071: F, t2709: F, t28142: F, t294: F, t296: F, t33980: F, t8459: F) -> (F,) {
    let t118633 = t109152 - t109154 - t109160 - t2709 * t1155 * t8459 / 16.0 + t109162 - t109165 - t294 * t2071 * t33980 / 8.0 + t110815 - t110817 + t111524 - t2709 * t296 * t28142 / 16.0;
    (t118633,)
}
