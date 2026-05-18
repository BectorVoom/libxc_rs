//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1336/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1336<F: Float>(t54166: F, t51256: F, t54158: F, t54160: F, t54162: F, t54164: F, t54168: F, t54170: F, t54173: F, t54175: F, t54177: F, t54179: F) -> F {
    let t55508 = F::new(7.0) / F::new(72.0) * t54166;
    let t55516 = -t54158 / F::new(24.0) - t54160 / F::new(12.0) - t54162 / F::new(96.0) + t54164 / F::new(48.0) + t55508 + t54168 / F::new(12.0) + t54170 / F::new(24.0) + F::new(7.0) / F::new(72.0) * t51256 - t54173 / F::new(48.0) + F::new(5.0) / F::new(96.0) * t54175 + t54177 / F::new(48.0) - t54179 / F::new(32.0);
    t55516
}
