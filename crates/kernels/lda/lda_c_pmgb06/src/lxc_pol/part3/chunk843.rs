//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 843/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk843<F: Float>(t2368: F, t754: F, t936: F, t97: F, t1786: F, t1789: F, t409: F, t328: F, t5915: F, t8483: F, t8527: F, t8529: F, t8536: F, t8538: F, t248: F, t4515: F, t686: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10984 = t2368 * t754 * t97 * t936;
    let t10985 = 0.41076328840066667 * t10984;
    let t10990 = t409 * t2368 * t1786 * t1789;
    let t10991 = 1.898172889849454 * t10990;
    let t10993 = t5915 * t328;
    let t10997 = 480.0 * t8483;
    let t10999 = 96.0 * t8527;
    let t11000 = 36.0 * t8529;
    let t11002 = 48.0 * t8536;
    let t11003 = 12.0 * t8538;
    let t11007 = t248 * t4515 * t686;
    (t10985, t10991, t10993, t10997, t10999, t11000, t11002, t11003, t11007)
}
