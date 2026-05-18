//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1355/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1355<F: Float>(t4083: F, t8743: F, t54616: F, t54621: F, t15084: F, t840: F, t14311: F, t14327: F, t14911: F, t2384: F, t2388: F, t2392: F, t2498: F, t51960: F, t51964: F, t51967: F, t54624: F, t54627: F, t8616: F) -> F {
    let t55884 = F::new(7.0) / F::new(144.0) * t8743 * t4083;
    let t55889 = F::new(7.0) / F::new(1152.0) * t54616;
    let t55892 = F::new(35.0) / F::new(216.0) * t54621;
    let t55901 = F::new(7.0) / F::new(144.0) * t840 * t15084;
    let t55903 = F::new(7.0) / F::new(144.0) * t51960 - t2384 * t14911 / F::new(96.0) + t55884 - t2388 * t14911 / F::new(96.0) - t2392 * t14911 / F::new(96.0) + t55889 - F::new(35.0) / F::new(576.0) * t51964 - F::new(35.0) / F::new(108.0) * t51967 - t55892 - t8616 * t4083 / F::new(96.0) - t2498 * t14311 / F::new(48.0) - t2498 * t14327 / F::new(48.0) - t54624 / F::new(24.0) + t55901 - t54627 / F::new(24.0);
    t55903
}
