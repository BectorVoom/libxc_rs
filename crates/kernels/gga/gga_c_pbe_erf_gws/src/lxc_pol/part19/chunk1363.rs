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
    let t58196 = t56505 / F::new(96.0) - t1115 * t54918 / F::new(48.0) + F::new(7.0) / F::new(288.0) * t58176 - F::new(35.0) / F::new(216.0) * t55192 + t55195 - t56511 / F::new(384.0) + t55198 - t56514 / F::new(12.0) - t8629 * t27047 * t3067 * t58050 * t938 / F::new(48.0) - t8629 * t55110 / F::new(24.0) + t56520 / F::new(768.0) - t8793 * t55204 / F::new(8.0) - t8793 * t55734 / F::new(12.0) - t56525 / F::new(768.0) - t56534 / F::new(384.0) - t56545 / F::new(192.0);
    t58196
}
