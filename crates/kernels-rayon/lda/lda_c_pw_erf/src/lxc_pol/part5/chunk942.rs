//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 942/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk942(t11371: f64, t1051: f64, t4393: f64, t1070: f64, t1799: f64, t8357: f64, t344: f64, t5685: f64, t1: f64, t1750: f64, t1755: f64, t2316: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11372 = 51.94726769812759_f64 * t11371;
    let t11373 = t4393 * t1051;
    let t11374 = 1.7544670192365612_f64 * t11373;
    let t11376 = 96.0_f64 * t1070 * t1799;
    let t11382 = 8.0_f64 * t8357;
    let t11387 = t344 * t5685;
    let t11388 = 24.0_f64 * t11387;
    let t11397 = t2316 * t1750 * t1 * t1755;
    (t11372, t11374, t11376, t11382, t11388, t11397)
}
