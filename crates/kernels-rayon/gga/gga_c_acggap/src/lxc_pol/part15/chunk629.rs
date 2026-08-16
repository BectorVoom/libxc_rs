//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 629/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk629(t6032: f64, t1711: f64, t229: f64, t2806: f64, t2812: f64, t2989: f64, t2995: f64, t5038: f64, t5044: f64, t6020: f64, t6023: f64, t6026: f64, t6027: f64, t6028: f64, t6029: f64, t6030: f64, t6031: f64) -> f64 {
    let t6033 = 4.0_f64 * t6032;
    let t6034 = t229 * t1711;
    let t6035 = 4.0_f64 * t6034;
    let t6036 = -t6020 - t2989 + t2806 - t2812 - t6023 - t6026 - t5038 + t2995 + t6027 + t6028 - t6029 - t6030 - t5044 - t6031 + t6033 - t6035;
    t6036
}
