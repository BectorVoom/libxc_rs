//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1237/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1237(t486: f64, t6596: f64, t4948: f64, t831: f64, t1499: f64, t2625: f64, t6616: f64, t12274: f64, t12276: f64, t12278: f64, t12281: f64, t132: f64, t1547: f64, t2583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16293 = t486 * t6596 / 15.0_f64;
    let t16294 = t831 * t4948;
    let t16295 = 4.0_f64 / 45.0_f64 * t16294;
    let t16297 = t1499 * t2625 / 30.0_f64;
    let t16298 = t486 * t6616;
    let t16299 = 2.0_f64 / 45.0_f64 * t16298;
    let t16300 = 2.0_f64 / 45.0_f64 * t12274;
    let t16301 = 4.0_f64 / 45.0_f64 * t12276;
    let t16302 = 2.0_f64 / 45.0_f64 * t12278;
    let t16303 = 2.0_f64 / 45.0_f64 * t12281;
    let t16305 = t132 * t1547 * t2583;
    (t16293, t16295, t16297, t16299, t16300, t16301, t16302, t16303, t16305)
}
