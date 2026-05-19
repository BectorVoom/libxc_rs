//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 348/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk348<F: Float>(t1701: F, t418: F, t560: F, t561: F, t1181: F, t1184: F) -> (F, F, F, F) {
    let t1702 = t418 * t1701;
    let t1703 = F::cast_from(2.9466786129040563_f64) * t560;
    let t1704 = F::cast_from(2.043763671738964_f64) * t561;
    let t1705 = -t1181 - t1703 + t1704 + t1184;
    (t1702, t1703, t1704, t1705)
}
