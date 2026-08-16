//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 865/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk865(t2619: f64, t9083: f64, t7939: f64, t8769: f64, t7921: f64, t1084: f64, t9253: f64, t2579: f64, t966: f64, t1038: f64, t8133: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9856 = t2619 * t9083;
    let t9857 = t9856 * t7939;
    let t9859 = t2619 * t8769;
    let t9860 = t9859 * t7921;
    let t9862 = t1084 * t9253;
    let t9863 = t2579 * t966;
    let t9864 = t1038 * t8133;
    let t9865 = t9863 * t9864;
    (t9857, t9860, t9862, t9863, t9864, t9865)
}
