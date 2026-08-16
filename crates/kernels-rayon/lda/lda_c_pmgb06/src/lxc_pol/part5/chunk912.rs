//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 912/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk912(t11227: f64, t8276: f64, t3615: f64, t63: f64, t370: f64, t38: f64, t8281: f64, t342: f64, t569: f64, t99: f64, t1271: f64, t2229: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11228 = t8276 * t11227;
    let t11230 = t63 * t3615;
    let t11234 = t38 * t370;
    let t11237 = t8281 * t11227;
    let t11303 = t99 * t569 * t342;
    let t11304 = t1271 * t2229 * t11303;
    (t11228, t11230, t11234, t11237, t11303, t11304)
}
