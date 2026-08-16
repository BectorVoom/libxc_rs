//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 483/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk483(t51: f64, t2524: f64, t2634: f64, t2673: f64, t2693: f64, t213: f64, t2146: f64, t555: f64, t1165: f64, t1166: f64, t1167: f64, t1169: f64, t1173: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t2695 = t2524 + t2634 + t2673 + t2693;
    let t2696 = t213 * t2695;
    let t2700 = piecewise3(t52, 0.0_f64, 2.0_f64 * t51 * t2146);
    let t2701 = t2700 * t555;
    let t2703 = t1165 + t1166 + t1167 + t1169 + t1173;
    (t2695, t2696, t2700, t2701, t2703)
}
