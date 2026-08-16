//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1037/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1037(t1869: f64, t8337: f64, t1525: f64, t1830: f64, t3103: f64, t2969: f64, t453: f64, t810: f64, t3010: f64, t4644: f64, t36: f64, t1069: f64, t4654: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12329 = t8337 * t1869;
    let t12332 = t1830 * t1525 * t3103;
    let t12335 = t1830 * t453 * t2969;
    let t12337 = t1830 * t810;
    let t12339 = t4644 * t3010;
    let t12341 = t36 * t1525 * t12339;
    let t12343 = t4654 * t1069;
    (t12329, t12332, t12335, t12337, t12339, t12341, t12343)
}
