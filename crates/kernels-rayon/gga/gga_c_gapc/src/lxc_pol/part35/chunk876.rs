//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 876/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk876(t3201: f64, t763: f64, t1051: f64, t2046: f64, t731: f64, t2155: f64, t2674: f64, t825: f64, t996: f64, t2255: f64, t493: f64, t1063: f64, t2221: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10172 = t763 * t3201;
    let t10174 = t2046 * t1051;
    let t10176 = t731 * t3201;
    let t10178 = t2155 * t1051;
    let t10180 = t2674 * t825;
    let t10181 = t996 * t10180;
    let t10182 = t493 * t2255;
    let t10183 = t10181 * t10182;
    let t10185 = t2221 * t1063;
    (t10172, t10174, t10176, t10178, t10183, t10185)
}
