//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 627/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk627(t2841: f64, t4057: f64, t4069: f64, t5474: f64, t87: f64, t40: f64, t2655: f64, t2658: f64, t2669: f64, t2695: f64, t2840: f64, t4044: f64, t4046: f64, t4049: f64, t4050: f64, t4061: f64, t4063: f64, t5479: f64) -> f64 {
    let t6005 = 8.0_f64 * t2841;
    let t6006 = 16.0_f64 * t4057;
    let t6007 = 2.0_f64 * t4069;
    let t6008 = t5474 * t87;
    let t6009 = t40 * t6008;
    let t6010 = t2655 - t2658 + t5479 + t2840 - t6005 + t4044 + t2669 + t2695 - t4046 - t4049 - t4050 - t6006 + t4061 - t4063 + t6007 + t6009;
    t6010
}
