//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 745/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk745(t1625: f64, t198: f64, t8820: f64, t1622: f64, t1043: f64, t1674: f64, t1013: f64, t1758: f64, t3079: f64, t561: f64, t1019: f64, t1776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8822 = t8820 * t198 * t1625;
    let t8823 = t1622 * t8822;
    let t8825 = t1043 * t1674;
    let t8830 = t1013 * t1758;
    let t8832 = t561 * t3079;
    let t8833 = t8832 * t1019;
    let t8835 = t1013 * t1776;
    (t8822, t8823, t8825, t8830, t8832, t8833, t8835)
}
