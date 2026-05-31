//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 914/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk914<F: Float>(t11373: F, t1238: F, t2210: F, t955: F, t1830: F, t2226: F, t2186: F, t1180: F, t776: F, t360: F, t5793: F, t947: F) -> (F, F, F, F, F, F, F) {
    let t11374 = F::cast_from(1.46904_f64) * t11373;
    let t11379 = t1238 * t2210 * t955;
    let t11380 = F::cast_from(0.9743416666666667_f64) * t11379;
    let t11388 = t2226 * t1830;
    let t11390 = t2186 * t1830;
    let t11392 = t1180 * t776;
    let t11393 = t360 * t11392;
    let t11395 = t5793 * t947;
    (t11374, t11380, t11388, t11390, t11392, t11393, t11395)
}
