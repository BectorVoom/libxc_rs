//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 886/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk886(t9214: f64, t9217: f64, t9220: f64, t9224: f64, t9226: f64, t9230: f64, t9233: f64, t9235: f64, t9239: f64, t9242: f64, t9250: f64, t9257: f64, t9263: f64) -> f64 {
    let t10752 = 0.17376185052903442709e-3_f64 * t9214 - 0.28960308421505737848e-5_f64 * t9217 - 0.51491428373437201896e-5_f64 * t9220 - 0.51491428373437201896e-5_f64 * t9224 + 0.21642471925239962898e-3_f64 * t9226 + 0.4048307291666666667e-4_f64 * t9230 + 0.20241536458333333336e-3_f64 * t9233 - 0.61320337121513228211e-3_f64 * t9235 - 0.22202903123154399017e-4_f64 * t9239 - 0.20241536458333333335e-4_f64 * t9242 - 0.29183437491952479975e-8_f64 * t9250 + 0.98481791311425691698e-7_f64 * t9257 + 0.1969635826228513834e-6_f64 * t9263;
    t10752
}
