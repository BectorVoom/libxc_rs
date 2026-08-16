//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 539/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk539(t1502: f64, t24: f64, t1111: f64, t1506: f64, t3104: f64, t1128: f64, t1508: f64, t1121: f64, t3117: f64, t123: f64, t438: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4314 = t24 * t1502;
    let t4315 = t1111 * t4314;
    let t4327 = t3104 * t1506;
    let t4333 = t1128 * t1508;
    let t4334 = t1121 * t4333;
    let t4336 = t3117 * t1506;
    let t4356 = t123 * t438;
    (t4314, t4315, t4327, t4333, t4334, t4336, t4356)
}
