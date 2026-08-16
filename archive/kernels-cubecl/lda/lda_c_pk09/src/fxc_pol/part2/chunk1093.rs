//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1093/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1093<F: Float>(t1872: F, t2758: F, t11101: F, t1827: F, t1800: F, t507: F, t1943: F, t11248: F, t1856: F, t11469: F, t1842: F, t1672: F, t2940: F) -> (F, F, F, F, F, F, F) {
    let t12014 = t1872 * t2758;
    let t12017 = t1827 * t11101;
    let t12018 = t12017 * t1800;
    let t12020 = t507 * t11101;
    let t12023 = t1943 * t2758;
    let t12026 = t1856 * t11248;
    let t12028 = t1842 * t11469;
    let t12030 = t2940 * t1672;
    (t12014, t12018, t12020, t12023, t12026, t12028, t12030)
}
