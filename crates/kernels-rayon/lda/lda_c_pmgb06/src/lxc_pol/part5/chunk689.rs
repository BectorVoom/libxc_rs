//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 689/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk689(t1983: f64, t6268: f64, t5187: f64, t806: f64, t2002: f64, t2007: f64, t1980: f64, t801: f64) -> (f64, f64, f64, f64) {
    let t6270 = 4.0_f64 / 45.0_f64 * t6268 * t1983;
    let t6272 = 2.0_f64 / 45.0_f64 * t5187 * t806;
    let t6274 = 2.0_f64 / 45.0_f64 * t2002 * t2007;
    let t6275 = t801 * t1980;
    (t6270, t6272, t6274, t6275)
}
