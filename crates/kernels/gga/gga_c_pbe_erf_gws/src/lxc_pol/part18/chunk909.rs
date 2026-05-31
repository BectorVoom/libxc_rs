//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 909/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk909<F: Float>(t10068: F, t133: F, t10071: F, t10065: F, t10094: F, t10096: F, t10102: F, t10106: F, t10123: F, t10126: F, t10129: F, t8238: F, t8249: F, t8252: F) -> F {
    let t10168 = t133 * t10068;
    let t10170 = t133 * t10071;
    let t10176 = t10094 - t10096 - F::cast_from(0.1724255e1_f64) * t10168 + F::cast_from(0.57475166666666666667e0_f64) * t10170 - F::cast_from(0.1724255e1_f64) * t133 * t10065 - F::cast_from(0.34485099999999999999e1_f64) * t8238 - t10102 + t8249 - F::cast_from(0.15326711111111111111e1_f64) * t8252 - t10106 - t10123 + t10126 + t10129;
    t10176
}
