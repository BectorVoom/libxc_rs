//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1265/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1265(t11249: f64, t1459: f64, t25176: f64, t11215: f64, t13676: f64, t13679: f64, t520: f64, t11216: f64, t13646: f64, t13654: f64, t35541: f64, t3948: f64) -> (f64, f64, f64, f64) {
    let t35643 = t25176 * t1459 * t11249;
    let t35647 = t11215 * t13676 * t520 * t13679;
    let t35650 = t11216 * t520 * t13646;
    let t35653 = t35541 * t3948 * t13654;
    (t35643, t35647, t35650, t35653)
}
