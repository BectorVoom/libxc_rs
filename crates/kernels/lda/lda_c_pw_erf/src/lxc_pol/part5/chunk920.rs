//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 920/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk920<F: Float>(t1210: F, t1638: F, t603: F, t1631: F, t4196: F, t4192: F, t4199: F, t10: F, t225: F, t4231: F, t602: F, t245: F, t4195: F) -> (F, F, F, F, F) {
    let t10697 = F::new(0.019878653761973935) * t1638 * t1210 * t603;
    let t10702 = t1631 * t4196;
    let t10704 = t4192 * t4199;
    let t10709 = F::new(0.4328416544945937) * t602 * t10 * t225 * t4231;
    let t10712 = F::new(0.06709045644666203) * t1638 * t245 * t4195;
    (t10697, t10702, t10704, t10709, t10712)
}
