//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 941/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk941<F: Float>(t8206: F, t339: F, t4405: F, t1034: F, t1798: F, t40: F, t3153: F, t748: F, t1765: F, t2987: F, t1055: F, t4393: F) -> (F, F, F, F, F, F) {
    let t11344 = F::cast_from(12.0_f64) * t8206;
    let t11348 = t339 * t4405;
    let t11349 = F::cast_from(12.0_f64) * t11348;
    let t11359 = t40 * t1798 * t1034;
    let t11360 = F::cast_from(3.0_f64) * t11359;
    let t11362 = t40 * t748 * t3153;
    let t11369 = t1765 * t2987;
    let t11371 = t4393 * t1055;
    (t11344, t11349, t11360, t11362, t11369, t11371)
}
