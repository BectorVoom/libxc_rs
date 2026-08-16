//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 906/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk906(t10984: f64, t1786: f64, t1789: f64, t2368: f64, t409: f64, t8483: f64, t8527: f64, t8529: f64, t8536: f64, t8538: f64, t4481: f64, t643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10985 = 0.41076328840066667_f64 * t10984;
    let t10990 = t409 * t2368 * t1786 * t1789;
    let t10991 = 1.898172889849454_f64 * t10990;
    let t10997 = 480.0_f64 * t8483;
    let t10999 = 96.0_f64 * t8527;
    let t11000 = 36.0_f64 * t8529;
    let t11002 = 48.0_f64 * t8536;
    let t11003 = 12.0_f64 * t8538;
    let t11065 = t643 * t4481;
    (t10985, t10991, t10997, t10999, t11000, t11002, t11003, t11065)
}
