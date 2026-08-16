//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1052/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1052<F: Float>(t6417: F, t6456: F, t9011: F, t9015: F, t9018: F, t9019: F, t9021: F, t9023: F, t9025: F, t9030: F, t9031: F, t9032: F) -> F {
    let t9536 = -t9011 - t9015 + t9018 - t9019 - t9021 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t6417 - t9023 - t9025 - t9030 + t9031 + t9032 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t6456;
    t9536
}
