//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1089/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1089<F: Float>(t9721: F, t9725: F, t9737: F, t9905: F, t493: F, t9946: F, t9909: F, t1508: F, t2134: F, t9923: F, t9925: F, t9928: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12740 = F::new(16.0) / F::new(27.0) * t9721;
    let t12741 = F::new(8.0) / F::new(27.0) * t9725;
    let t12742 = F::new(32.0) / F::new(45.0) * t9737;
    let t12743 = F::new(8.0) / F::new(15.0) * t9905;
    let t12745 = F::new(4.0) / F::new(5.0) * t493 * t9946;
    let t12746 = F::new(16.0) / F::new(135.0) * t9909;
    let t12747 = t1508 * t2134;
    let t12748 = F::new(4.0) / F::new(15.0) * t12747;
    let t12749 = F::new(4.0) / F::new(45.0) * t9923;
    let t12750 = F::new(16.0) / F::new(45.0) * t9925;
    let t12751 = F::new(4.0) / F::new(15.0) * t9928;
    (t12740, t12741, t12742, t12743, t12745, t12746, t12748, t12749, t12750, t12751)
}
