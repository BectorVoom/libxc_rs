//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1100/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1100<F: Float>(t16605: F, t436: F, t7465: F, t1928: F, t2592: F, t12832: F, t20209: F, t20210: F, t20211: F, t20213: F, t20215: F, t20219: F, t20221: F, t9770: F) -> (F, F, F, F) {
    let t20222 = t16605 / F::new(15.0);
    let t20223 = t7465 * t436;
    let t20224 = t20223 / F::new(45.0);
    let t20225 = t2592 * t1928;
    let t20226 = t20225 / F::new(15.0);
    let t20227 = -t12832 + t20209 + t20210 - t20211 - t9770 - t20213 + t20215 + t20219 - t20221 - t20222 + t20224 + t20226;
    (t20222, t20224, t20226, t20227)
}
