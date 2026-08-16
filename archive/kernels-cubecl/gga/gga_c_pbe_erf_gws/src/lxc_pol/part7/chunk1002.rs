//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1002/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1002<F: Float>(t17432: F, t17434: F, t17436: F, t17439: F, t17443: F, t17448: F, t17450: F, t17452: F, t17456: F, t17461: F, t17463: F, t17465: F) -> F {
    let t18282 = t17432 + t17434 + t17436 + t17439 + t17443 - t17448 + t17450 - t17452 - t17456 - t17461 + t17463 + t17465;
    t18282
}
