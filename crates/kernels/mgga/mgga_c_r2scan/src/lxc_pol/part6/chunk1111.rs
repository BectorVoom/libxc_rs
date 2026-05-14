//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1111/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1111<F: Float>(t19890: F, t6085: F, t6087: F, t6064: F, t6093: F, t2097: F, t2105: F, t254: F, t265: F, t6077: F, t8: F, t9: F, t537: F, t6243: F, t19879: F, t2155: F) -> (F, F, F, F, F) {
    let t19892 = t6085 * t19890 * t6087;
    let t19895 = t6093 * t19890 * t6064;
    let t19904 = 0.36021350028521610017e1 * t254 * t2097 / t9 / t6077 / t8 * t265 * t2105;
    let t19905 = t6243 * t537;
    let t19907 = t2155 * t19905 * t19879;
    (t19892, t19895, t19904, t19905, t19907)
}
