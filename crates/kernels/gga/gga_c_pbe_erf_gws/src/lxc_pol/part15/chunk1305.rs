//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1305/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1305<F: Float>(t54107: F, t54109: F, t54111: F, t54114: F, t54115: F, t54118: F, t54120: F, t54122: F, t54124: F, t54126: F, t54129: F, t54130: F) -> F {
    let t54132 = t54107 / F::new(96.0) - t54109 / F::new(48.0) + t54111 / F::new(192.0) + t54114 - t54115 / F::new(192.0) + t54118 + t54120 / F::new(48.0) - t54122 / F::new(48.0) + t54124 / F::new(192.0) + F::new(119.0) / F::new(3456.0) * t54126 - t54129 + t54130 / F::new(96.0);
    t54132
}
