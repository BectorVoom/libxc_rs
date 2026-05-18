//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1071/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1071<F: Float>(t4702: F, t518: F, t577: F, t3416: F, t5356: F, t1472: F, t5371: F, t1454: F, t5327: F, t1462: F, t1325: F, t1440: F, t2181: F, t3464: F) -> (F, F, F, F, F, F) {
    let t12541 = t4702 * t518;
    let t12543 = F::new(4.0) / F::new(15.0) * t12541 * t577;
    let t12545 = F::new(4.0) / F::new(5.0) * t3416 * t5356;
    let t12546 = t1472 * t5371;
    let t12547 = F::new(16.0) / F::new(15.0) * t12546;
    let t12549 = F::new(4.0) / F::new(15.0) * t5327 * t1454;
    let t12551 = F::new(4.0) / F::new(9.0) * t5327 * t1462;
    let t12555 = F::new(4.0) / F::new(15.0) * t1325 * t1440 * t2181 * t3464;
    (t12543, t12545, t12547, t12549, t12551, t12555)
}
