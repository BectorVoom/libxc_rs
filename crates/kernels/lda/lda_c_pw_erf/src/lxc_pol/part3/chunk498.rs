//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 498/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk498<F: Float>(t1527: F, t565: F, t835: F, t331: F, t830: F, t1371: F, t1944: F, t1949: F, t589: F, t1210: F, t21: F) -> (F, F, F, F, F, F) {
    let t2044 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1527;
    let t2046 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t565 * t835;
    let t2053 = t331 * t830;
    let t2055 = t1371 * t1944;
    let t2058 = t589 * t1949;
    let t2061 = t21 * t1210;
    (t2044, t2046, t2053, t2055, t2058, t2061)
}
