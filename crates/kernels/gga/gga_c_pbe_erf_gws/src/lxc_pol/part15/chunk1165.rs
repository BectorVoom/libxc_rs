//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1165/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1165<F: Float>(t3062: F, t3959: F, t14128: F, t14130: F, t14777: F, t14779: F, t14782: F, t14785: F, t14788: F, t14793: F, t14800: F, t14803: F, t14806: F, t14809: F, t2408: F, t3066: F) -> F {
    let t14812 = t3959 * t3062;
    let t14814 = t14777 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14779 - t14782 / F::cast_from(96.0_f64) - t14785 / F::cast_from(384.0_f64) - t14788 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14128 - t3066 * t14793 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14130 + t14800 / F::cast_from(1536.0_f64) - t2408 * t14803 / F::cast_from(24.0_f64) + t14806 / F::cast_from(48.0_f64) + t2408 * t14809 / F::cast_from(48.0_f64) + t14812 / F::cast_from(48.0_f64);
    t14814
}
