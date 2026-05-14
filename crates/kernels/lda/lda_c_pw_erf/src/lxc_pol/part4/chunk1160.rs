//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1160/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1160<F: Float>(t12714: F, t17053: F, t17055: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F, t17069: F, t17073: F, t17074: F, t17078: F, t17080: F, t17081: F, t17082: F, t17086: F, t17089: F) -> (F,) {
    let t17090 = t17053 + t17055 - t17057 + t17059 + t17061 + t17064 - t17066 + 0.002206740740740741 * t12714 + t17069 + t17073 - t17074 + t17078 - t17080 - t17081 + t17082 - t17086 + t17089;
    (t17090,)
}
