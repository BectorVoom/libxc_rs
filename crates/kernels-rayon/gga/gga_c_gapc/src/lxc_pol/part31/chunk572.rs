//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 572/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk572(t1096: f64, t972: f64, t1055: f64, t644: f64, t311: f64, t442: f64, t906: f64) -> (f64, f64, f64, f64) {
    let t3268 = t1096 * t972;
    let t3271 = t1055 * t644;
    let t3272 = t311 * t3271;
    let t3273 = t442 * t906;
    (t3268, t3271, t3272, t3273)
}
