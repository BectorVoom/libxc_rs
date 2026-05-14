//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 950/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk950<F: Float>(t110: F, t360: F, t5766: F, t11316: F, t5770: F, t5779: F, t1234: F, t30: F, t348: F, t780: F, t8228: F, t5783: F, t2217: F, t410: F, t365: F, t5740: F, t5772: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11320 = t360 * t110 * t5766;
    let t11322 = t5770 * t11316;
    let t11330 = t360 * t110 * t5779;
    let t11334 = t30 * t110 * t1234;
    let t11335 = t348 * t780 * t11334;
    let t11341 = t5770 * t8228;
    let t11343 = t5783 * t8228;
    let t11354 = t360 * t410 * t2217;
    let t11357 = t365 * t5740 * t5772;
    (t11320, t11322, t11330, t11334, t11335, t11341, t11343, t11354, t11357)
}
