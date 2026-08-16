//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 660/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk660(t110: f64, t141: f64, t106: f64, t317: f64, t1659: f64, t282: f64, t115: f64, t2770: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t3843 = t141 * t110;
    let t3860 = t106 * t317;
    let t3881 = t1659 * t282;
    let t3882 = t2770 * t115;
    let t3883 = t3882 * t5;
    (t3843, t3860, t3881, t3882, t3883)
}
