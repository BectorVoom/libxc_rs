//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 659/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk659(t304: f64, t5009: f64, t5834: f64, t1642: f64, t5838: f64, t1623: f64, t5039: f64, t5161: f64, t5045: f64, t5190: f64, t5208: f64, t5212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5878 = t304 * t5009;
    let t5880 = t5878 * t5834 / 3.0_f64;
    let t5884 = t1642 * t5838 / 9.0_f64;
    let t5886 = t1623 * t5838 / 9.0_f64;
    let t5903 = 0.3056501876701794_f64 * t5039;
    let t5907 = 2.0430704068275776_f64 * t5161;
    let t5916 = 0.2037667917801196_f64 * t5045;
    let t5917 = 0.17025586723563146_f64 * t5190;
    let t5922 = 1.5323028051206833_f64 * t5208;
    let t5923 = 1.5323028051206833_f64 * t5212;
    (t5880, t5884, t5886, t5903, t5907, t5916, t5917, t5922, t5923)
}
