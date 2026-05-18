//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1334/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1334<F: Float>(t54113: F, t54117: F, t54126: F, t54128: F, t54107: F, t54109: F, t54111: F, t54115: F, t54120: F, t54122: F, t54124: F, t54130: F) -> F {
    let t55480 = F::new(7.0) / F::new(144.0) * t54113;
    let t55482 = F::new(7.0) / F::new(144.0) * t54117;
    let t55486 = F::new(119.0) / F::new(1728.0) * t54126;
    let t55487 = F::new(7.0) / F::new(288.0) * t54128;
    let t55489 = t54107 / F::new(48.0) - t54109 / F::new(24.0) + t54111 / F::new(96.0) + t55480 - t54115 / F::new(96.0) + t55482 + t54120 / F::new(24.0) - t54122 / F::new(24.0) + t54124 / F::new(96.0) + t55486 - t55487 + t54130 / F::new(48.0);
    t55489
}
