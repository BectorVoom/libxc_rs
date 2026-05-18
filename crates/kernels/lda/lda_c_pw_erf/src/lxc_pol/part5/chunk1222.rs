//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1222/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1222<F: Float>(t12874: F, t1308: F, t1325: F, t1326: F, t13551: F, t15614: F, t16121: F, t18133: F, t2146: F, t2385: F, t34: F, t3794: F, t4738: F, t4763: F, t4804: F, t4829: F, t4841: F, t571: F, t6256: F, t6263: F, t6285: F, t6357: F, t6401: F, t6455: F, t739: F, t743: F, t7809: F) -> F {
    let t22082 = F::new(16.0) / F::new(15.0) * t2146 * t6401 - F::new(4.0) / F::new(15.0) * t571 * t1308 * t16121 * t743 + F::new(8.0) / F::new(15.0) * t571 * t4841 * t6285 * t34 + F::new(16.0) / F::new(15.0) * t15614 * t2385 + F::new(16.0) / F::new(15.0) * t12874 * t2385 + F::new(32.0) / F::new(15.0) * t4738 * t6455 + F::new(8.0) / F::new(15.0) * t4804 * t7809 + F::new(8.0) / F::new(15.0) * t3794 * t7809 + F::new(8.0) / F::new(15.0) * t1325 * t1326 * t18133 * t739 + F::new(16.0) / F::new(15.0) * t1325 * t4829 * t6263 * t34 + t13551 + F::new(8.0) / F::new(15.0) * t4763 * t6256 - F::new(4.0) / F::new(15.0) * t2146 * t6357;
    t22082
}
