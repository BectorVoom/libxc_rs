//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 912/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk912(t1190: f64, t1191: f64, t275: f64, t9637: f64, t9680: f64, t2460: f64, t4875: f64, t2459: f64, t4878: f64, t4821: f64, t1179: f64, t2140: f64) -> (f64, f64, f64, f64) {
    let t9683 = t1191 * t275 * t9680 + t1190 * t9637;
    let t9689 = 1.28_f64 * t4875 * t2460;
    let t9690 = t2459 * t4878;
    let t9692 = 1.28_f64 * t4821 * t9690;
    let t9695 = t1179 * t2140;
    (t9683, t9689, t9692, t9695)
}
