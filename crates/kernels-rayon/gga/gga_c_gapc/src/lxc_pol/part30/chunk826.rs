//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 826/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk826(t9843: f64, t9846: f64, t7259: f64, t9067: f64, t8142: f64, t1084: f64, t9282: f64, t3415: f64, t2619: f64, t9083: f64, t7939: f64, t8769: f64) -> (f64, f64, f64, f64, f64) {
    let t9847 = t9843 * t9846;
    let t9849 = t7259 * t9067;
    let t9850 = t9849 * t8142;
    let t9852 = t1084 * t9282;
    let t9853 = t9852 * t3415;
    let t9856 = t2619 * t9083;
    let t9857 = t9856 * t7939;
    let t9859 = t2619 * t8769;
    (t9847, t9850, t9853, t9857, t9859)
}
