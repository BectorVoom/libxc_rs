//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1055/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1055<F: Float>(t3232: F, t6627: F, t6502: F, t6506: F, t6508: F, t6517: F, t9084: F, t9086: F, t9090: F, t9094: F, t9096: F, t9100: F, t9101: F) -> F {
    let t9565 = F::new(7.0) / F::new(288.0) * t6627 * t3232;
    let t9567 = -F::new(7.0) / F::new(768.0) * t6502 - F::new(119.0) / F::new(1728.0) * t6506 + F::new(7.0) / F::new(1152.0) * t6508 - t9084 + t9086 + t9090 + t9094 - t9096 - t9565 + t9100 - t9101 + F::new(7.0) / F::new(2304.0) * t6517;
    t9567
}
