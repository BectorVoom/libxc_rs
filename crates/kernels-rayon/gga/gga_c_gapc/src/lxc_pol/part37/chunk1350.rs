//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1350/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1350(t10086: f64, t1125: f64, t2469: f64, t1096: f64, t11039: f64, t12291: f64, t7063: f64, t972: f64, t1112: f64, t24906: f64, t10786: f64, t2964: f64) -> (f64, f64, f64, f64, f64) {
    let t36262 = 2.0_f64 * t2469 * t1125 * t10086;
    let t36266 = 2.0_f64 * t2469 * t11039 * t1096;
    let t36269 = 12.0_f64 * t7063 * t12291 * t972;
    let t36270 = t24906 * t1112;
    let t36271 = t2964 * t10786;
    (t36262, t36266, t36269, t36270, t36271)
}
