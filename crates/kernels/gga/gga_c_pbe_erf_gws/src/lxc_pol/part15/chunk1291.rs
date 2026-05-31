//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1291/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1291<F: Float>(t3959: F, t8797: F, t14121: F, t8624: F, t14001: F, t14463: F, t2409: F, t2417: F, t3066: F, t4182: F, t53950: F, t53953: F, t53959: F, t53963: F, t53966: F, t53968: F, t53971: F, t53973: F, t53976: F, t53977: F, t53980: F, t9296: F) -> F {
    let t53981 = t3959 * t8797;
    let t53983 = t14121 * t8624;
    let t53985 = t14001 * t14463;
    let t53986 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53985;
    let t53987 = t53950 / F::cast_from(24.0_f64) + t53953 - t3066 * t2409 * t9296 * t4182 * t2417 / F::cast_from(16.0_f64) + F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t53959 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t53963 - t53966 / F::cast_from(48.0_f64) + t53968 / F::cast_from(24.0_f64) - t53971 + t53973 / F::cast_from(16.0_f64) + t53976 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t53977 + t53980 + t53981 / F::cast_from(24.0_f64) + t53983 / F::cast_from(8.0_f64) + t53986;
    t53987
}
