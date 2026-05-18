//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1327/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1327<F: Float>(t3038: F, t497: F, t5068: F, t6561: F, t15274: F, t529: F, t6559: F, t1586: F, t6560: F, t16825: F, t5077: F, t5084: F) -> (F, F, F, F) {
    let t17444 = F::new(8.0) / F::new(45.0) * t5068 * t3038 * t497 * t6561;
    let t17448 = F::new(8.0) / F::new(45.0) * t5068 * t6559 * t15274 * t529;
    let t17452 = F::new(4.0) / F::new(45.0) * t5068 * t6559 * t6560 * t1586;
    let t17455 = F::new(4.0) / F::new(15.0) * t5077 * t5084 * t16825;
    (t17444, t17448, t17452, t17455)
}
