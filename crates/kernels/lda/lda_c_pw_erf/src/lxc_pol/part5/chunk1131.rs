//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1131/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1131<F: Float>(t2171: F, t6489: F, t6493: F, t6353: F, t6443: F, t4804: F, t7688: F, t3794: F, t1325: F, t1326: F, t6557: F, t784: F) -> (F, F, F, F, F, F, F) {
    let t20947 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2171 * t6489;
    let t20949 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t2171 * t6493;
    let t20951 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2171 * t6353;
    let t20953 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t2171 * t6443;
    let t20955 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4804 * t7688;
    let t20957 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3794 * t7688;
    let t20961 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1325 * t1326 * t6557 * t784;
    (t20947, t20949, t20951, t20953, t20955, t20957, t20961)
}
