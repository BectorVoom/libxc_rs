//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1003/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1003<F: Float>(t5381: F, t588: F, t97: F, t1981: F, t4852: F, t5447: F, t4866: F, t5463: F, t2983: F, t493: F, t5486: F, t2933: F, t6747: F) -> (F, F, F, F, F) {
    let t11930 = t5381 * t97 * t588;
    let t11934 = F::new(8.0) / F::new(15.0) * t1981 * t5447 * t4852;
    let t11937 = F::new(4.0) / F::new(9.0) * t1981 * t5463 * t4866;
    let t11940 = t493 * t5486 * t2983 / F::new(15.0);
    let t11943 = F::new(2.0) / F::new(15.0) * t493 * t6747 * t2933;
    (t11930, t11934, t11937, t11940, t11943)
}
