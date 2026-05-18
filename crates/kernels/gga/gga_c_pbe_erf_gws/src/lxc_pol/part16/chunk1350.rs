//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1350/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1350<F: Float>(t53970: F, t53975: F, t53985: F, t54427: F, t54429: F, t14918: F, t2220: F, t2388: F, t2392: F, t335: F, t338: F, t4228: F, t51864: F, t51877: F, t52514: F, t52525: F, t53973: F, t53981: F, t53983: F) -> F {
    let t55739 = F::new(7.0) / F::new(72.0) * t53970;
    let t55741 = F::new(7.0) / F::new(288.0) * t53975;
    let t55745 = F::new(7.0) / F::new(36.0) * t53985;
    let t55751 = F::new(119.0) / F::new(1728.0) * t54427;
    let t55752 = F::new(7.0) / F::new(72.0) * t54429;
    let t55758 = -t55739 + t53973 / F::new(8.0) + t55741 + t53981 / F::new(12.0) + t53983 / F::new(4.0) + F::new(7.0) / F::new(288.0) * t52514 + t55745 - t335 * t338 * t2220 * t4228 / F::new(96.0) - F::new(7.0) / F::new(36.0) * t51864 - t55751 + t55752 - t52525 + F::new(35.0) / F::new(108.0) * t51877 - t2388 * t14918 / F::new(96.0) - t2392 * t14918 / F::new(96.0);
    t55758
}
