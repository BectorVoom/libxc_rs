//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1002/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1002(t18497: f64, t4140: f64, t4139: f64, t312: f64, t5225: f64, t684: f64, t10492: f64, t15370: f64, t4176: f64, t15369: f64, t4635: f64, t875: f64) -> (f64, f64, f64, f64) {
    let t19513 = t4140 * t18497;
    let t19514 = t4139 * t19513;
    let t19517 = t312 * t5225;
    let t19518 = t19517 * t684;
    let t19519 = t10492 * t19518;
    let t19522 = t15370 * t4176;
    let t19523 = t15369 * t19522;
    let t19526 = t4635 * t875;
    (t19514, t19519, t19523, t19526)
}
