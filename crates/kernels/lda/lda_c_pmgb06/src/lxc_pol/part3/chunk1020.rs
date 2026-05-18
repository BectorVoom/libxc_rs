//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1020/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1020<F: Float>(t2002: F, t2971: F, t3303: F, t10255: F, t153: F, t1859: F, t439: F, t4659: F, t5253: F, t10247: F, t4645: F, t2010: F, t4655: F) -> (F, F, F, F, F, F) {
    let t12129 = F::new(2.0) / F::new(15.0) * t2002 * t2971;
    let t12131 = t2002 * t3303 / F::new(9.0);
    let t12135 = t439 * t10255 * t153 * t1859 / F::new(9.0);
    let t12138 = t439 * t5253 * t4659 / F::new(9.0);
    let t12139 = t10247 * t153;
    let t12142 = F::new(8.0) / F::new(27.0) * t439 * t12139 * t4645;
    let t12145 = F::new(4.0) / F::new(9.0) * t2010 * t5253 * t4655;
    (t12129, t12131, t12135, t12138, t12142, t12145)
}
