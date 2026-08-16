//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 914/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk914(t44: f64, t1202: f64, t2463: f64, t276: f64, t9683: f64, t9708: f64, t2468: f64, t4875: f64, t2467: f64, t4910: f64, t4821: f64, t1179: f64, t2146: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t9711 = piecewise3(t45, t9683, t1202 * t2463 + t276 * t9708);
    let t9717 = 1.28_f64 * t4875 * t2468;
    let t9718 = t2467 * t4910;
    let t9720 = 1.28_f64 * t4821 * t9718;
    let t9723 = t1179 * t2146;
    (t9711, t9717, t9720, t9723)
}
