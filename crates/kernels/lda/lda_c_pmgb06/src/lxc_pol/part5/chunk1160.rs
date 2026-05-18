//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1160/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1160<F: Float>(t12012: F, t2496: F, t493: F, t5486: F, t6390: F, t1981: F, t6394: F, t6755: F, t6747: F, t6760: F, t20919: F, t20920: F, t20922: F, t20925: F, t20929: F, t20931: F) -> (F, F, F, F, F, F) {
    let t20934 = F::new(2.0) / F::new(15.0) * t493 * t12012 * t2496;
    let t20937 = F::new(2.0) / F::new(15.0) * t493 * t5486 * t6390;
    let t20940 = F::new(4.0) / F::new(15.0) * t1981 * t5486 * t6394;
    let t20943 = t493 * t5486 * t6755 / F::new(15.0);
    let t20946 = F::new(2.0) / F::new(15.0) * t493 * t6747 * t6760;
    let t20947 = -t20919 + t20920 - t20922 - t20925 - t20929 - t20931 - t20934 - t20937 + t20940 - t20943 - t20946;
    (t20934, t20937, t20940, t20943, t20946, t20947)
}
