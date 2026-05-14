//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1023/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1023<F: Float>(t29274: F, t3332: F, t7614: F, t1060: F, t269: F, t783: F, t9083: F, t12550: F, t788: F, t3308: F, t6449: F, t8807: F, t10776: F, t8826: F, t3295: F, t9160: F) -> (F, F, F, F, F, F) {
    let t43072 = t7614 * t3332 * t29274;
    let t43076 = t783 * t9083 * t269 * t1060;
    let t43079 = t783 * t12550 * t788;
    let t43083 = t6449 * t3308 * t8807;
    let t43086 = t10776 * t3308 * t8826;
    let t43088 = t3295 * t9160;
    (t43072, t43076, t43079, t43083, t43086, t43088)
}
