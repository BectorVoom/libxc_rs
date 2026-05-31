//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1363/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1363<F: Float>(t15513: F, t840: F, t1115: F, t27047: F, t3067: F, t54918: F, t55110: F, t55192: F, t55195: F, t55198: F, t55204: F, t55734: F, t56505: F, t56511: F, t56514: F, t56520: F, t56525: F, t56534: F, t56545: F, t58050: F, t8629: F, t8793: F, t938: F) -> F {
    let t58176 = t840 * t15513;
    let t58196 = t56505 / F::cast_from(96.0_f64) - t1115 * t54918 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t58176 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t55192 + t55195 - t56511 / F::cast_from(384.0_f64) + t55198 - t56514 / F::cast_from(12.0_f64) - t8629 * t27047 * t3067 * t58050 * t938 / F::cast_from(48.0_f64) - t8629 * t55110 / F::cast_from(24.0_f64) + t56520 / F::cast_from(768.0_f64) - t8793 * t55204 / F::cast_from(8.0_f64) - t8793 * t55734 / F::cast_from(12.0_f64) - t56525 / F::cast_from(768.0_f64) - t56534 / F::cast_from(384.0_f64) - t56545 / F::cast_from(192.0_f64);
    t58196
}
