//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1332/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1332<F: Float>(t54052: F, t54072: F, t54043: F, t54045: F, t54048: F, t54057: F, t54059: F, t54061: F, t54063: F, t54065: F, t54067: F, t54069: F) -> F {
    let t55452 = F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t54052;
    let t55460 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54072;
    let t55461 = t54043 / F::cast_from(12.0_f64) + t54045 / F::cast_from(192.0_f64) + t54048 / F::cast_from(32.0_f64) - t55452 - t54057 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t54059 + t54061 / F::cast_from(48.0_f64) + t54063 / F::cast_from(192.0_f64) - t54065 / F::cast_from(96.0_f64) + t54067 / F::cast_from(96.0_f64) - t54069 / F::cast_from(16.0_f64) + t55460;
    t55461
}
