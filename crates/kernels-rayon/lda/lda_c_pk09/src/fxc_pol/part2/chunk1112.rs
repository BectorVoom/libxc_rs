//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1112/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1112(t12296: f64, t2956: f64, t2957: f64, t2958: f64, t2959: f64, t4: f64, t7566: f64) -> f64 {
    let tv2rho21 = t12296 * t4 + t2956 + t2957 + t2958 + 2.0_f64 * t2959 + 2.0_f64 * t7566;
    tv2rho21
}
