//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1398/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1398<F: Float>(t4227: F, t8589: F, t829: F, t830: F, t54619: F, t54621: F, t55889: F, t55901: F, t57639: F, t57641: F, t57643: F, t57648: F, t57652: F, t57654: F, t57657: F, t57661: F, t57663: F, t57666: F, t827: F) -> F {
    let t58896 = t8589 * t4227;
    let t58898 = t829 * t830 * t58896;
    let t58902 = t55889 - t54619 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t54621 - t57639 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57641 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57643 - t57648 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t57652 + t57654 / F::cast_from(12.0_f64) + t57657 / F::cast_from(24.0_f64) + t57661 / F::cast_from(24.0_f64) + t57663 / F::cast_from(48.0_f64) - t827 * t58898 / F::cast_from(48.0_f64) - t57666 / F::cast_from(48.0_f64) + t55901;
    t58902
}
