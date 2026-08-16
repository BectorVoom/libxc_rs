//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 605/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk605<F: Float>(t4298: F, t4299: F, t1759: F, t707: F, t1763: F, t1183: F, t301: F, t398: F, t297: F, t122: F, t4182: F, t302: F) -> (F, F, F, F, F, F, F) {
    let t4301 = F::cast_from(4.569219094474146e-06_f64) * t4298 * t4299;
    let t4304 = t707 * t1759;
    let t4307 = F::cast_from(0.05987117005127304_f64) * t707 * t1763;
    let t4317 = t398 * t1183 * t301;
    let t4318 = t297 * t4317;
    let t4320 = t122 * t4182;
    let t4322 = F::cast_from(0.19513566535229734_f64) * t4320 * t302;
    (t4301, t4304, t4307, t4317, t4318, t4320, t4322)
}
