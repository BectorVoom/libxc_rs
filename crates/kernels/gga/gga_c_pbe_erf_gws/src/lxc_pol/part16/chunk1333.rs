//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1333/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1333<F: Float>(t54087: F, t54094: F, t54102: F, t51244: F, t54075: F, t54077: F, t54080: F, t54082: F, t54085: F, t54092: F, t54096: F, t54098: F) -> F {
    let t55467 = F::new(7.0) / F::new(72.0) * t54087;
    let t55469 = F::new(35.0) / F::new(216.0) * t54094;
    let t55473 = F::new(7.0) / F::new(36.0) * t54102;
    let t55474 = -t54075 / F::new(24.0) + t54077 / F::new(384.0) - t54080 / F::new(24.0) + t54082 / F::new(24.0) - t54085 / F::new(24.0) + t55467 - t54092 / F::new(6.0) + t55469 - t54096 / F::new(384.0) + t54098 / F::new(64.0) - F::new(7.0) / F::new(144.0) * t51244 + t55473;
    t55474
}
