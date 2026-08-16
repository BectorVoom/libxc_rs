//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1356/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1356<F: Float>(t1105: F, t14311: F, t15081: F, t2376: F, t2408: F, t2409: F, t3921: F, t52989: F, t54911: F, t54915: F, t54923: F, t54927: F, t56166: F, t56168: F, t56170: F, t56174: F, t56176: F, t56181: F, t56190: F, t56194: F) -> F {
    let t58011 = t52989 - t56166 / F::cast_from(768.0_f64) - t56168 / F::cast_from(12.0_f64) + t54911 + t56170 / F::cast_from(4.0_f64) + t54915 - t56174 / F::cast_from(768.0_f64) + t56176 / F::cast_from(12.0_f64) - t3921 * t14311 / F::cast_from(96.0_f64) + t56181 / F::cast_from(384.0_f64) + t2408 * t2409 * t2376 * t15081 * t1105 / F::cast_from(24.0_f64) + t54923 - t56190 / F::cast_from(24.0_f64) - t56194 / F::cast_from(192.0_f64) - t54927;
    t58011
}
