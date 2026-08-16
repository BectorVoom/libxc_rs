//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1230/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1230(t10577: f64, t4354: f64, t2257: f64, t4042: f64, t10544: f64, t10558: f64, t10570: f64, t10578: f64, t10594: f64, t1312: f64, t1316: f64, t14567: f64, t14570: f64, t14571: f64, t14575: f64, t14587: f64, t14593: f64, t346: f64, t4013: f64, t4045: f64, t4231: f64, t4398: f64, t4414: f64, t5583: f64, t6018: f64, t790: f64) -> f64 {
    let t14596 = t10577 * t4354;
    let t14601 = t2257 * t4042;
    let t14606 = -18.0_f64 * t5583 * t10578 + 0.05987117005127304_f64 * t14567 + t14570 + 0.05987117005127304_f64 * t14571 + 0.0001639671923854359_f64 * t14575 + 6.0_f64 * t1316 * t1312 * t4414 - 6.0_f64 * t346 * t4398 * t4013 + 3.0_f64 * t1316 * t790 * t10558 + 6.0_f64 * t4231 * t14587 - 18.0_f64 * t6018 * t10570 - 3.0_f64 * t4231 * t14593 - 6.0_f64 * t4231 * t14596 + 18.0_f64 * t6018 * t10544 + 6.0_f64 * t346 * t14601 * t4045 - 0.054045904796391424_f64 * t10594;
    t14606
}
