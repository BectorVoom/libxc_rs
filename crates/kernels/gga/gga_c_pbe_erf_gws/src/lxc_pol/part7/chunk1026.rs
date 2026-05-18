//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1026/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1026<F: Float>(t18582: F, t18535: F, t18537: F, t18539: F, t18541: F, t18556: F, t18562: F, t18567: F, t18571: F, t18574: F, t18577: F, t18580: F) -> (F, F) {
    let t18583 = F::new(0.65061485296689145287e-1) * t18582;
    let t18584 = -t18535 + t18537 + t18539 - t18541 - t18556 - t18562 + t18567 + t18571 - t18574 + t18577 + t18580 + t18583;
    (t18583, t18584)
}
