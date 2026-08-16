//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1326/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1326(t19395: f64, t19401: f64, t19424: f64, t19438: f64, t19514: f64, t24559: f64, t24564: f64, t24571: f64, t24578: f64, t24587: f64, t24601: f64, t24617: f64, t24633: f64, t24643: f64, t24654: f64, t24655: f64, t4055: f64, t4057: f64, t4060: f64, t4062: f64, t4065: f64, t4069: f64, t4101: f64, t5395: f64, t6009: f64, t6013: f64, t6581: f64, t6585: f64, t7: f64) -> f64 {
    let t24662 = 12.0_f64 * t6581 - 6.0_f64 * t4055 - 48.0_f64 * t4057 + 8.0_f64 * t4060 - 32.0_f64 * t4062 + 2.0_f64 * t4065 + 6.0_f64 * t4069 + 2.0_f64 * t6009 + 2.0_f64 * t6013 + 6.0_f64 * t4101 + 2.0_f64 * t5395 + t7 * (t19395 + t19401 + t19424 + t19438 + t19514 + t24559 + t24564 + t24571 + t24578 + t24587 + t24601 + t24617 + t24633 + t24643 + t24654 + t24655) - 6.0_f64 * t6585;
    t24662
}
