//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1013/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1013<F: Float>(t27: F, t6067: F, t693: F, t1112: F, t6078: F, t248: F, t686: F, t14935: F, t285: F, t8590: F, t8594: F, t8598: F, t8603: F, t8610: F, t8612: F, t8614: F, t8616: F, t8621: F, t8626: F, t8629: F, t8633: F, t8637: F) -> (F,) {
    let t14971 = t6067 * t27 * t693;
    let t14973 = t6078 * t1112;
    let t14977 = t248 * t6067 * t686;
    let t14981 = -24.0 * t8590 - t8594 - t8598 + t8603 + t8610 - t8612 - 160.0 * t8614 - 0.0003662289461201309 * t14971 + 0.00024415263074675396 * t14973 + 20.0 * t8616 + t8621 - t8626 + 2.0 * t14977 + t248 * t14935 * t285 - t8629 - t8633 - t8637;
    (t14981,)
}
