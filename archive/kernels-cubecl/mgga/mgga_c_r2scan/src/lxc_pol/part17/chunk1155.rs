//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1155/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1155<F: Float>(t10776: F, t3308: F, t8795: F, t10772: F, t8799: F, t3105: F, t37764: F, t10781: F, t9513: F, t574: F, t9147: F, t1054: F, t2139: F, t8752: F) -> (F, F, F, F, F, F) {
    let t42999 = t10776 * t3308 * t8795;
    let t43002 = t10772 * t3308 * t8799;
    let t43004 = t37764 * t3105;
    let t43009 = t10781 * t9513;
    let t43012 = t574 * t3308 * t9147;
    let t43015 = t2139 * t1054 * t8752;
    (t42999, t43002, t43004, t43009, t43012, t43015)
}
