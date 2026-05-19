//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 855/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk855<F: Float>(t4296: F, t4300: F, t2994: F, t2999: F, t3008: F, t3015: F, t3155: F, t14: F, t2: F, t41: F, t174: F, t2824: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8120 = F::cast_from(7.302458460456296_f64) * t4296;
    let t8121 = F::cast_from(12.654485932329694_f64) * t4300;
    let t8122 = F::cast_from(14.03573615389249_f64) * t2994;
    let t8123 = F::cast_from(207.78907079251036_f64) * t2999;
    let t8126 = F::cast_from(0.0022787712934626155_f64) * t3008;
    let t8130 = F::cast_from(0.013780452414814815_f64) * t3015;
    let t8134 = F::new(4.0) * t3155;
    let t8138 = F::new(1.0) / t14 / t2 / t41 / F::new(48.0);
    let t8141 = t8138 * t2 * t2824 * t174;
    (t8120, t8121, t8122, t8123, t8126, t8130, t8134, t8138, t8141)
}
