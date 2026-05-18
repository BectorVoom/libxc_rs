//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1096/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1096<F: Float>(t2010: F, t6155: F, t6498: F, t1385: F, t439: F, t477: F, t7497: F, t1897: F, t19802: F, t1972: F, t6791: F, t16513: F, t1907: F) -> (F, F, F, F, F) {
    let t20182 = F::new(4.0) / F::new(9.0) * t2010 * t6498 * t6155;
    let t20186 = t439 * t1385 * t7497 * t477 / F::new(45.0);
    let t20189 = F::new(2.0) / F::new(45.0) * t439 * t1897 * t19802;
    let t20191 = F::new(2.0) / F::new(15.0) * t1972 * t6791;
    let t20194 = t439 * t16513 * t1907 / F::new(15.0);
    (t20182, t20186, t20189, t20191, t20194)
}
