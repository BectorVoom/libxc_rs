//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 670/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk670<F: Float>(t1487: F, t4062: F, t571: F, t2973: F, t575: F, t574: F, t1498: F, t595: F, t1496: F, t202: F, t184: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4063 = t4062 * t1487;
    let t4064 = t571 * t4063;
    let t4065 = F::new(8.0) / F::new(27.0) * t4064;
    let t4066 = t575 * t2973;
    let t4067 = t574 * t4066;
    let t4069 = F::new(4.0) / F::new(45.0) * t571 * t4067;
    let t4071 = F::new(2.0) / F::new(5.0) * t1498 * t595;
    let t4072 = t202 * t1496;
    let t4073 = t4072 * t184;
    (t4063, t4064, t4065, t4066, t4067, t4069, t4071, t4072, t4073)
}
