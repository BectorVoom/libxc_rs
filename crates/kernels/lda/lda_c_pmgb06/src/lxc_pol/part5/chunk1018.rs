//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1018/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1018<F: Float>(t1447: F, t7715: F, t1420: F, t15180: F, t15182: F, t15184: F, t15189: F, t2010: F, t2948: F, t439: F, t5482: F, t6146: F, t6185: F, t6189: F, t6375: F, t6494: F, t6498: F, t7554: F, t7555: F) -> F {
    let t19178 = t1447 * t7715;
    let t19181 = F::new(2.0) / F::new(15.0) * t439 * t5482 * t6375 + F::new(2.0) / F::new(5.0) * t439 * t6494 * t6185 - F::new(2.0) / F::new(3.0) * t439 * t6498 * t6146 - F::new(8.0) / F::new(15.0) * t2010 * t6494 * t6189 - t1420 * t7555 / F::new(15.0) - t439 * t2948 * t7554 / F::new(15.0) - F::new(4.0) / F::new(45.0) * t15180 - F::new(8.0) / F::new(45.0) * t15182 - F::new(4.0) / F::new(45.0) * t15184 + F::new(4.0) / F::new(45.0) * t19178 - F::new(2.0) / F::new(45.0) * t15189;
    t19181
}
