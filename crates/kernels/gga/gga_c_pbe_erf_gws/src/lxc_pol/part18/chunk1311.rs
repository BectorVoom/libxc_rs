//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1311/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1311<F: Float>(t14627: F, t15139: F, t2408: F, t2409: F, t26604: F, t51572: F, t53704: F, t53726: F, t53728: F, t56740: F, t56743: F, t56745: F, t56747: F, t56753: F, t56757: F, t56761: F, t56769: F, t56773: F, t8589: F) -> F {
    let t56775 = -t56740 / F::new(96.0) - t56743 / F::new(96.0) + F::new(7.0) / F::new(288.0) * t56745 - F::new(7.0) / F::new(2304.0) * t56747 - F::new(35.0) / F::new(432.0) * t51572 - t53704 + t56753 / F::new(768.0) + t56757 / F::new(768.0) - t56761 / F::new(3072.0) + t2408 * t2409 * t8589 * t14627 / F::new(24.0) + t26604 * t15139 / F::new(96.0) + F::new(7.0) / F::new(36.0) * t56769 + t56773 / F::new(96.0) - t53726 + t53728;
    t56775
}
