//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 641/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk641<F: Float>(t1835: F, t495: F, t2065: F, t435: F, t132: F, t2015: F, t432: F, t1423: F, t1908: F, t1898: F, t1902: F, t1447: F, t1925: F) -> (F, F, F, F, F, F, F, F) {
    let t5312 = t495 * t1835;
    let t5326 = t435 * t2065;
    let t5328 = F::new(2.0) / F::new(45.0) * t132 * t5326;
    let t5330 = F::new(2.0) / F::new(45.0) * t432 * t2015;
    let t5342 = F::new(4.0) / F::new(135.0) * t1423 * t1908;
    let t5349 = F::new(8.0) / F::new(135.0) * t1423 * t1898;
    let t5354 = F::new(4.0) / F::new(81.0) * t1423 * t1902;
    let t5356 = F::new(4.0) / F::new(135.0) * t1447 * t1925;
    (t5312, t5326, t5328, t5330, t5342, t5349, t5354, t5356)
}
