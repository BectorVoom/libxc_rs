//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1121/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1121(t2002: f64, t6491: f64, t6495: f64, t12006: f64, t2623: f64, t493: f64, t529: f64, t851: f64, t10711: f64, t10714: f64, t13088: f64, t20451: f64, t20452: f64, t20454: f64, t20456: f64, t20460: f64, t20463: f64) -> (f64, f64, f64, f64) {
    let t20465 = 2.0_f64 / 15.0_f64 * t2002 * t6491;
    let t20467 = 4.0_f64 / 15.0_f64 * t2002 * t6495;
    let t20472 = 3.0_f64 / 5.0_f64 * t493 * t12006 * t2623 * t851 * t529;
    let t20473 = t20451 + t13088 - t20452 - t20454 + t10711 + t10714 - t20456 - t20460 + t20463 - t20465 - t20467 - t20472;
    (t20465, t20467, t20472, t20473)
}
