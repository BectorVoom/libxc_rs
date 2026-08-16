//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 804/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk804(t44: f64, t5430: f64, t131: f64, t178: f64, t1848: f64, t513: f64, t1491: f64, t831: f64, t1512: f64, t815: f64, t1831: f64, t529: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5431 = t5430 * t44;
    let t5432 = t5431 * t131;
    let t5434 = t5432 * t178 / 30.0_f64;
    let t5436 = t1848 * t513 / 15.0_f64;
    let t5438 = t831 * t1491 / 30.0_f64;
    let t5440 = t1512 * t815 / 30.0_f64;
    let t5441 = t1831 * t529;
    (t5431, t5432, t5434, t5436, t5438, t5440, t5441)
}
