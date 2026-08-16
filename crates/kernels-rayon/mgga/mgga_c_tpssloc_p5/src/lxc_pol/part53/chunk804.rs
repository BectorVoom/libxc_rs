//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 804/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk804(t4072: f64, t88: f64, t1453: f64, t22470: f64, t666: f64, t22473: f64, t4067: f64, t6530: f64, t1982: f64, t8944: f64) -> (f64, f64, f64, f64, f64) {
    let t26117 = t88 * t4072;
    let t26127 = t22470 * t1453;
    let t26129 = t1453 * t666;
    let t26130 = t22473 * t26129;
    let t26132 = t6530 * t4067;
    let t26161 = t1982 * t8944;
    (t26117, t26127, t26130, t26132, t26161)
}
