//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 131/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk131<F: Float>(t311: F, t156: F, t31: F, t4: F, t27: F, t13: F, t14: F, t1: F, t119: F, t155: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t312 = 1.0 / t311;
    let t315 = t4 * t156 * t31;
    let t316 = 0.0011073577833333333 * t315;
    let t317 = t27 * t27;
    let t318 = 1.0 / t317;
    let t319 = t13 * t318;
    let t320 = 1.0 / t14;
    let t321 = t320 * t1;
    let t322 = t119 * t155;
    let t323 = t321 * t322;
    let t325 = t4 * t156;
    (t312, t316, t317, t318, t319, t320, t321, t322, t323, t325)
}
