//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 724/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk724(t1343: f64, t3076: f64, t7553: f64, t7765: f64, t13819: f64, t7757: f64, t1985: f64, t3814: f64, t14224: f64, t7229: f64, t3114: f64, t3124: f64, t70186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70383 = t3076 * t1343;
    let t70385 = t7553 * t70383 * t7765;
    let t70387 = t13819 * t7757;
    let t70397 = t1985 * t3814;
    let t70423 = t7229 * t14224;
    let t70439 = t3114 * t70186 * t3124;
    (t70383, t70385, t70387, t70397, t70423, t70439)
}
