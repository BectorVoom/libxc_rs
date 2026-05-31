//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1159/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1159<F: Float>(t16762: F, t16765: F, t16768: F, t16819: F, t16829: F, t16874: F, t12197: F, t12310: F, t21249: F, t21251: F, t21255: F, t21257: F, t21261: F) -> (F, F, F, F, F, F, F) {
    let t21262 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t16762;
    let t21263 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t16765;
    let t21264 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t16768;
    let t21265 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16819;
    let t21266 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16829;
    let t21267 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16874;
    let t21268 = -t21249 + t21251 - t21255 - t21257 - t21261 + t21262 + t21263 - t21264 + t12197 + t12310 + t21265 - t21266 + t21267;
    (t21262, t21263, t21264, t21265, t21266, t21267, t21268)
}
