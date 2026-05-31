//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 952/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk952<F: Float>(t219: F, t4048: F, t3589: F, t2114: F, t4564: F, t1529: F, t1960: F, t1466: F, t3667: F, t1401: F, t3899: F, t3476: F, t5146: F) -> (F, F, F, F, F, F) {
    let t11913 = t4048 * t219;
    let t11914 = t11913 * t3589;
    let t11946 = t2114 * t4564;
    let t11947 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t11946;
    let t11954 = t1960 * t1529;
    let t11955 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t11954;
    let t11983 = t1466 * t3667;
    let t11989 = t3899 * t1401;
    let t12025 = t5146 * t3476;
    (t11914, t11947, t11955, t11983, t11989, t12025)
}
