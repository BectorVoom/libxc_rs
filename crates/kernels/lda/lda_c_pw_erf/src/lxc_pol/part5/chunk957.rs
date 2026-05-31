//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 957/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk957<F: Float>(t1390: F, t1458: F, t1245: F, t4561: F, t565: F, t3704: F, t4487: F, t3518: F, t3892: F, t529: F, t4722: F, t1251: F, t4489: F) -> (F, F, F, F, F, F, F) {
    let t12321 = t1458 * t1390;
    let t12322 = t12321 * t1245;
    let t12356 = t565 * t4561;
    let t12357 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12356;
    let t12362 = t4487 * t3704;
    let t12380 = t3892 * t529 * t3518;
    let t12387 = t4722 * t1245;
    let t12403 = t4489 * t1251;
    (t12321, t12322, t12357, t12362, t12380, t12387, t12403)
}
