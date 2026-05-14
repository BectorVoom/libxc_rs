//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 794/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk794<F: Float>(t9214: F, t9217: F, t9220: F, t9224: F, t9226: F, t9230: F, t9233: F, t9235: F, t9239: F, t9242: F, t9250: F, t9257: F, t9263: F, t9273: F, t9279: F, t9284: F, t9289: F, t9292: F, t9295: F, t9299: F, t9302: F, t9309: F, t9312: F, t9315: F, t9318: F, t9320: F) -> (F, F) {
    let t10752 = 0.17376185052903442709e-3 * t9214 - 0.28960308421505737848e-5 * t9217 - 0.51491428373437201896e-5 * t9220 - 0.51491428373437201896e-5 * t9224 + 0.21642471925239962898e-3 * t9226 + 0.4048307291666666667e-4 * t9230 + 0.20241536458333333336e-3 * t9233 - 0.61320337121513228211e-3 * t9235 - 0.22202903123154399017e-4 * t9239 - 0.20241536458333333335e-4 * t9242 - 0.29183437491952479975e-8 * t9250 + 0.98481791311425691698e-7 * t9257 + 0.1969635826228513834e-6 * t9263;
    let t10767 = -0.91682472831214851819e-8 * t9273 - 0.10129555677746642575e-5 * t9279 - 0.49522272202316919253e-5 * t9284 + 0.33765185592488808582e-6 * t9289 + 0.67530371184977617164e-6 * t9292 - 0.20241536458333333335e-4 * t9295 + 0.10136107947527008247e-3 * t9299 + 0.13900948042322754167e-3 * t9302 - 0.33765185592488808582e-6 * t9309 - 0.24761136101158459626e-5 * t9312 + 0.34752370105806885418e-3 * t9315 + 0.34752370105806885418e-3 * t9318 + 0.2318836277704281739e-4 * t9320;
    (t10752, t10767)
}
