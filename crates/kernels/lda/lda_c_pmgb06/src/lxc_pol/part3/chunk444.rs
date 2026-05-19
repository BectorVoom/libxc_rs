//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 444/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk444<F: Float>(t1773: F, t302: F, t1316: F, t1317: F, t1323: F, t1324: F, t1655: F, t1748: F, t1750: F, t1753: F, t1755: F, t1760: F, t1765: F, t1772: F, t295: F, t297: F, t346: F, t388: F, t61: F) -> (F, F) {
    let t1775 = F::cast_from(0.05321881782335382_f64) * t1773 * t302;
    let t1776 = F::new(6.0) * t1316 * t388 * t1317 - t346 * t1323 * t1324 + t1655 * t61 + t1748 * t295 + F::cast_from(0.039914113367515366_f64) * t1750 + t1753 - F::cast_from(0.01197423401025461_f64) * t297 * t1755 - F::cast_from(0.02394846802050922_f64) * t1760 - t1765 - t1772 - t1775;
    (t1775, t1776)
}
