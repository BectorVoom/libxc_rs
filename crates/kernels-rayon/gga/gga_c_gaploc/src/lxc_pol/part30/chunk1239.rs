//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1239/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1239(t1843: f64, t21476: f64, t25289: f64, t25462: f64, t2558: f64, t9647: f64, t25055: f64, t5539: f64, t16880: f64, t25059: f64, t1854: f64, t22008: f64, t32348: f64) -> (f64, f64, f64, f64, f64) {
    let t32584 = t21476 * t1843 * t25289;
    let t32585 = 0.64087718584518535698e-3_f64 * t32584;
    let t32587 = t9647 * t25462 * t2558;
    let t32588 = 0.64087718584518535698e-3_f64 * t32587;
    let t32590 = t9647 * t5539 * t25055;
    let t32591 = 0.38452631150711121418e-2_f64 * t32590;
    let t32593 = t9647 * t16880 * t25059;
    let t32594 = 0.19226315575355560709e-2_f64 * t32593;
    let t32597 = 0.17090058289204942853e-2_f64 * t22008 * t32348 * t1854;
    (t32585, t32588, t32591, t32594, t32597)
}
