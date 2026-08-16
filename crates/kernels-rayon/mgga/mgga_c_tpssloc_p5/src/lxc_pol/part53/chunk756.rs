//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 756/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk756(t1878: f64, t244: f64, t2230: f64, t6589: f64, t213: f64, t6593: f64, t229: f64, t6546: f64, t805: f64, t243: f64, t598: f64, t6584: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23063 = t23062 * t6593;
    let t23069 = t6546 * t229;
    let t23070 = t23069 * t805;
    let t23071 = 7.0_f64 / 72.0_f64 * t23070;
    let t23075 = t243 * t243;
    let t23076 = 1.0_f64 / t23075;
    let t23077 = t598 * t23076;
    let t23083 = t6584 * t6604;
    (t23056, t23062, t23063, t23069, t23071, t23077, t23083)
}
