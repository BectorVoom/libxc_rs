//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 872/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk872<F: Float>(t2760: F, t2943: F, t2949: F, t2988: F, t4296: F, t4300: F, t2994: F, t2999: F, t3008: F, t3015: F, t3155: F, t14: F, t2: F, t41: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8110 = F::new(48.0) * t2760;
    let t8113 = F::new(14.03573615389249) * t2943;
    let t8114 = F::new(415.5781415850207) * t2949;
    let t8118 = F::new(4101.558808403118) * t2988;
    let t8120 = F::new(7.302458460456296) * t4296;
    let t8121 = F::new(12.654485932329694) * t4300;
    let t8122 = F::new(14.03573615389249) * t2994;
    let t8123 = F::new(207.78907079251036) * t2999;
    let t8126 = F::new(0.0022787712934626155) * t3008;
    let t8130 = F::new(0.013780452414814815) * t3015;
    let t8134 = F::new(4.0) * t3155;
    let t8138 = F::new(1.0) / t14 / t2 / t41 / F::new(48.0);
    (t8110, t8113, t8114, t8118, t8120, t8121, t8122, t8123, t8126, t8130, t8134, t8138)
}
