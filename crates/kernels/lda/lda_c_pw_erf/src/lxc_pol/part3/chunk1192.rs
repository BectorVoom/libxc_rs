//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1192/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1192<F: Float>(t3828: F, t3974: F, t4475: F, t4521: F, t811: F, t3833: F, t34: F, t3975: F, t1309: F, t13115: F, t3619: F, t6748: F) -> (F, F, F, F) {
    let t14029 = F::new(8.0) / F::new(15.0) * t3974 * t4475 * t3828;
    let t14030 = t4521 * t811;
    let t14033 = F::new(8.0) / F::new(9.0) * t3974 * t14030 * t3833;
    let t14034 = t3975 * t34;
    let t14037 = F::new(16.0) / F::new(15.0) * t13115 * t14034 * t1309;
    let t14040 = F::new(16.0) / F::new(15.0) * t3974 * t6748 * t3619;
    (t14029, t14033, t14037, t14040)
}
