//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1276/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1276<F: Float>(t1134: F, t13917: F, t53799: F, t824: F, t938: F, t15156: F, t9270: F, t15351: F, t22534: F, t2409: F, t3066: F, t3068: F, t4182: F, t50949: F, t52962: F, t52969: F, t52971: F, t52973: F, t52992: F, t56166: F, t56168: F, t56170: F, t56174: F, t56176: F, t6126: F, t9283: F) -> F {
    let t56181 = t13917 * t53799 * t824 * t1134 * t938;
    let t56183 = t9270 * t15156;
    let t56189 = -t3066 * t9283 * t6126 * t4182 * t3068 / F::new(8.0) - t52962 + t52969 + t52971 + t52973 + F::new(119.0) / F::new(6912.0) * t50949 - t56166 / F::new(1536.0) - t56168 / F::new(24.0) + t56170 / F::new(8.0) - t56174 / F::new(1536.0) + t56176 / F::new(24.0) + t56181 / F::new(768.0) - F::new(7.0) / F::new(144.0) * t56183 - t3066 * t2409 * t22534 * t15351 / F::new(16.0) + t52992;
    t56189
}
