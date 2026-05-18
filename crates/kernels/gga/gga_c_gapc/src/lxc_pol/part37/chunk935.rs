//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 935/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk935<F: Float>(t9214: F, t9217: F, t9220: F, t9224: F, t9226: F, t9230: F, t9233: F, t9235: F, t9239: F, t9242: F, t9250: F, t9257: F, t9263: F) -> F {
    let t10752 = F::new(0.17376185052903442709e-3) * t9214 - F::new(0.28960308421505737848e-5) * t9217 - F::new(0.51491428373437201896e-5) * t9220 - F::new(0.51491428373437201896e-5) * t9224 + F::new(0.21642471925239962898e-3) * t9226 + F::new(0.4048307291666666667e-4) * t9230 + F::new(0.20241536458333333336e-3) * t9233 - F::new(0.61320337121513228211e-3) * t9235 - F::new(0.22202903123154399017e-4) * t9239 - F::new(0.20241536458333333335e-4) * t9242 - F::new(0.29183437491952479975e-8) * t9250 + F::new(0.98481791311425691698e-7) * t9257 + F::new(0.1969635826228513834e-6) * t9263;
    t10752
}
