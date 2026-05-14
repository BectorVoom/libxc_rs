//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 794/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk794<F: Float>(t4296: F, t4300: F, t2994: F, t2999: F, t3008: F, t3015: F, t3155: F, t14: F, t2: F, t41: F, t174: F, t2824: F, t2716: F, t343: F, t1191: F, t732: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8120 = 7.302458460456296 * t4296;
    let t8121 = 12.654485932329694 * t4300;
    let t8122 = 14.03573615389249 * t2994;
    let t8123 = 207.78907079251036 * t2999;
    let t8126 = 0.0022787712934626155 * t3008;
    let t8130 = 0.013780452414814815 * t3015;
    let t8134 = 4.0 * t3155;
    let t8138 = 1.0 / t14 / t2 / t41 / 48.0;
    let t8141 = t8138 * t2 * t2824 * t174;
    let t8143 = t2716 * t343;
    let t8145 = t732 * t1191;
    (t8120, t8121, t8122, t8123, t8126, t8130, t8134, t8138, t8141, t8143, t8145)
}
