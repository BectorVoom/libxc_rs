//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 542/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk542(t54: f64, t623: f64, t48: f64, t633: f64, t3223: f64, t810: f64, t3290: f64, t664: f64, t673: f64, t662: f64) -> (f64, f64, f64, f64, f64) {
    let t3344 = t623 * t54;
    let t3348 = t48 * t633;
    let t3368 = t810 * t3223;
    let t3371 = 19.489173774580152_f64 * t810 * t3290;
    let t3383 = t673 * t664 * t623;
    let t3384 = t662 * t3383;
    (t3344, t3348, t3368, t3371, t3384)
}
