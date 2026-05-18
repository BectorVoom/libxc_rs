//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1342/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1342<F: Float>(t14692: F, t3979: F, t51967: F, t2410: F, t4164: F, t51952: F, t51954: F, t51957: F, t51958: F, t51960: F, t51964: F, t54588: F, t54593: F, t54596: F, t54598: F, t54599: F, t54605: F, t54607: F, t54613: F) -> F {
    let t54616 = t3979 * t14692;
    let t54617 = F::new(7.0) / F::new(2304.0) * t54616;
    let t54619 = F::new(35.0) / F::new(216.0) * t51967;
    let t54620 = -t54588 / F::new(768.0) - t54593 / F::new(384.0) - t54596 / F::new(48.0) + t54598 * t54599 * t4164 * t2410 / F::new(4.0) - F::new(5.0) / F::new(384.0) * t54605 - t54607 / F::new(96.0) + F::new(7.0) / F::new(72.0) * t51952 + F::new(7.0) / F::new(1152.0) * t51954 + t51957 - F::new(7.0) / F::new(288.0) * t51958 + t54613 / F::new(48.0) + F::new(7.0) / F::new(288.0) * t51960 + t54617 - F::new(35.0) / F::new(1152.0) * t51964 - t54619;
    t54620
}
