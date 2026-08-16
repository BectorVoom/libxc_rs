//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 807/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk807(t69618: f64, t8450: f64, t69621: f64, t14225: f64, t3352: f64, t9164: f64, t3140: f64, t8675: f64, t13868: f64, t11683: f64, t14236: f64, t14243: f64, t2078: f64) -> (f64, f64, f64, f64) {
    let t74535 = t8450 * t69618;
    let t74536 = t74535 * t69621;
    let t74539 = t14225 * t3352 * t9164;
    let t74548 = t8675 * t3140;
    let t74549 = t74548 * t13868;
    let t74553 = t14236 * t14243 * t2078 * t11683;
    (t74536, t74539, t74549, t74553)
}
