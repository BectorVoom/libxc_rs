//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1048/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1048(t203: f64, t6393: f64, t579: f64, t599: f64, t585: f64, t9419: f64, t1433: f64, t129: f64, t15481: f64) -> (f64, f64, f64, f64, f64) {
    let t20572 = t203 * t6393;
    let t20592 = t579 * t599;
    let t20669 = t585 * t9419;
    let t20670 = t1433 * t20669;
    let t20671 = t15481 * t129;
    (t20572, t20592, t20669, t20670, t20671)
}
