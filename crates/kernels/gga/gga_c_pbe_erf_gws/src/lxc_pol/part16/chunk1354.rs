//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1354/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1354<F: Float>(t54566: F, t51952: F, t51954: F, t51958: F, t52562: F, t54564: F, t54572: F, t54575: F, t54581: F, t54588: F, t54593: F, t54596: F, t54605: F, t54607: F, t54613: F) -> F {
    let t55863 = F::new(7.0) / F::new(36.0) * t54566;
    let t55877 = -t54564 / F::new(48.0) + t55863 + t54572 / F::new(24.0) - t54575 / F::new(24.0) + F::new(7.0) / F::new(288.0) * t52562 - t54581 / F::new(16.0) - t54588 / F::new(384.0) - t54593 / F::new(192.0) - t54596 / F::new(24.0) - F::new(5.0) / F::new(192.0) * t54605 - t54607 / F::new(48.0) + F::new(7.0) / F::new(36.0) * t51952 + F::new(7.0) / F::new(576.0) * t51954 - F::new(7.0) / F::new(144.0) * t51958 + t54613 / F::new(24.0);
    t55877
}
