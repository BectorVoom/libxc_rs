//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 895/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk895(t1067: f64, t2337: f64, t1047: f64, t4459: f64, t4461: f64, t4475: f64, t8617: f64, t8621: f64, t8859: f64, t8863: f64, t8867: f64, t8871: f64, t9443: f64, t9446: f64, t9449: f64, t9453: f64, t9459: f64, t98: f64) -> f64 {
    let t9461 = t2337 * t1067;
    let t9467 = -t9443 / 9.0_f64 - t9446 * t98 / 6.0_f64 - t9449 * t98 / 6.0_f64 - t4459 + t4461 + t9453 * t8617 / 3.0_f64 + t8621 * t1047 / 36.0_f64 - 0.14975624337724558_f64 * t4475 + t9459 / 9.0_f64 - t9461 / 9.0_f64 - 0.01233429741534199_f64 * t8859 + 0.01233429741534199_f64 * t8863 + 0.01233429741534199_f64 * t8867 - 0.14975624337724558_f64 * t8871;
    t9467
}
