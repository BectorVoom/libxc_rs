//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1329/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1329<F: Float>(t53873: F, t53886: F, t51771: F, t52417: F, t52483: F, t53846: F, t53848: F, t53856: F, t53862: F, t53867: F, t53870: F, t53876: F, t53878: F, t53880: F, t53884: F, t8793: F) -> F {
    let t55403 = F::new(7.0) / F::new(576.0) * t53873;
    let t55408 = F::new(119.0) / F::new(3456.0) * t53886;
    let t55409 = -F::new(7.0) / F::new(1152.0) * t51771 + t53846 / F::new(12.0) + t53848 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t52483 + t53856 / F::new(192.0) + t8793 * t52417 / F::new(24.0) + t53862 / F::new(96.0) + F::new(5.0) / F::new(96.0) * t53867 - t53870 / F::new(768.0) + t55403 - t53876 / F::new(128.0) - t53878 / F::new(12.0) + t53880 / F::new(8.0) + t53884 / F::new(48.0) + t55408;
    t55409
}
