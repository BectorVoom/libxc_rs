//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1078/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1078<F: Float>(t3794: F, t5378: F, t5382: F, t1336: F, t5334: F, t2146: F, t3716: F, t1472: F, t4901: F, t2143: F, t3709: F, t1446: F, t4907: F) -> (F, F, F, F, F, F, F) {
    let t12621 = t3794 * t5378;
    let t12622 = F::new(16.0) / F::new(15.0) * t12621;
    let t12624 = F::new(4.0) / F::new(5.0) * t3794 * t5382;
    let t12626 = F::new(8.0) / F::new(15.0) * t5334 * t1336;
    let t12628 = F::new(8.0) / F::new(9.0) * t2146 * t3716;
    let t12629 = t1472 * t4901;
    let t12630 = F::new(8.0) / F::new(9.0) * t12629;
    let t12631 = t3709 * t2143;
    let t12632 = F::new(8.0) / F::new(45.0) * t12631;
    let t12633 = t1446 * t4907;
    (t12622, t12624, t12626, t12628, t12630, t12632, t12633)
}
