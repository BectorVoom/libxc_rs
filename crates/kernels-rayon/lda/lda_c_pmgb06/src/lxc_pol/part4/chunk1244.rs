//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1244/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1244(t16382: f64, t446: f64, t12517: f64, t1080: f64, t6560: f64, t5068: f64, t5139: f64, t12556: f64, t10693: f64, t10696: f64, t10697: f64, t10699: f64, t16372: f64, t16373: f64, t16374: f64, t16375: f64, t16379: f64, t16381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16383 = t16382 * t446;
    let t16384 = 4.0_f64 / 135.0_f64 * t16383;
    let t16385 = 8.0_f64 / 81.0_f64 * t12517;
    let t16386 = t6560 * t1080;
    let t16389 = 4.0_f64 / 15.0_f64 * t5068 * t5139 * t16386;
    let t16390 = 16.0_f64 / 135.0_f64 * t12556;
    let t16394 = -t16372 - t16373 - t16374 - t16375 - t16379 + t16381 + t16384 - t16385 - t16389 + t16390 + 0.002206740740740741_f64 * t10693 + t10696 + 8.0_f64 / 3.0_f64 * t10697 + 8.0_f64 * t10699;
    (t16384, t16385, t16386, t16389, t16390, t16394)
}
