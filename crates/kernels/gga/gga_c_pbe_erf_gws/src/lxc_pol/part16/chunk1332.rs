//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1332/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1332<F: Float>(t54052: F, t54072: F, t54043: F, t54045: F, t54048: F, t54057: F, t54059: F, t54061: F, t54063: F, t54065: F, t54067: F, t54069: F) -> F {
    let t55452 = F::new(7.0) / F::new(96.0) * t54052;
    let t55460 = F::new(7.0) / F::new(72.0) * t54072;
    let t55461 = t54043 / F::new(12.0) + t54045 / F::new(192.0) + t54048 / F::new(32.0) - t55452 - t54057 / F::new(4.0) - F::new(5.0) / F::new(96.0) * t54059 + t54061 / F::new(48.0) + t54063 / F::new(192.0) - t54065 / F::new(96.0) + t54067 / F::new(96.0) - t54069 / F::new(16.0) + t55460;
    t55461
}
