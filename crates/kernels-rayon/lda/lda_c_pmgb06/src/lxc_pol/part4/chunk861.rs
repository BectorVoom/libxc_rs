//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 861/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk861(t12: f64, t2386: f64, t3922: f64, t1079: f64, t2389: f64, t1072: f64, t14: f64, t2133: f64, t337: f64, t5974: f64, t257: f64, t6053: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t6054 = t3922 * t2386;
    let t6059 = t1079 * t2389;
    let t6065 = piecewise3(t13, 0.0_f64, -8.0_f64 / 27.0_f64 * t6054 * t337 - 16.0_f64 / 9.0_f64 * t2133 * t1072 + 4.0_f64 / 9.0_f64 * t6059 * t337 + 4.0_f64 / 3.0_f64 * t14 * t5974);
    let t6067 = (t6053 + t6065) * t257;
    (t6054, t6059, t6067)
}
