//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1329/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1329<F: Float>(t53873: F, t53886: F, t51771: F, t52417: F, t52483: F, t53846: F, t53848: F, t53856: F, t53862: F, t53867: F, t53870: F, t53876: F, t53878: F, t53880: F, t53884: F, t8793: F) -> F {
    let t55403 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53873;
    let t55408 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t53886;
    let t55409 = -F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51771 + t53846 / F::cast_from(12.0_f64) + t53848 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52483 + t53856 / F::cast_from(192.0_f64) + t8793 * t52417 / F::cast_from(24.0_f64) + t53862 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t53867 - t53870 / F::cast_from(768.0_f64) + t55403 - t53876 / F::cast_from(128.0_f64) - t53878 / F::cast_from(12.0_f64) + t53880 / F::cast_from(8.0_f64) + t53884 / F::cast_from(48.0_f64) + t55408;
    t55409
}
