//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1357/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1357<F: Float>(t15492: F, t8801: F, t11407: F, t12220: F, t14327: F, t14881: F, t15482: F, t2376: F, t2408: F, t2409: F, t3066: F, t3921: F, t52159: F, t53015: F, t54937: F, t54942: F, t54946: F, t55729: F, t56197: F, t56206: F, t56209: F, t56236: F, t56240: F, t56242: F, t810: F, t9283: F) -> F {
    let t58028 = t8801 * t15492;
    let t58035 = F::new(35.0) / F::new(108.0) * t53015 - t56197 / F::new(96.0) - t3066 * t9283 * t14881 * t11407 / F::new(8.0) - t54937 - t54942 - t56206 / F::new(192.0) + t56209 / F::new(384.0) - t3921 * t14327 / F::new(96.0) - t54946 + t2408 * t2409 * t2376 * t15482 * t810 / F::new(48.0) - t56236 / F::new(6.0) + F::new(7.0) / F::new(48.0) * t58028 + t56240 / F::new(768.0) + F::new(35.0) / F::new(216.0) * t52159 - t12220 * t55729 / F::new(96.0) + F::new(7.0) / F::new(576.0) * t56242;
    t58035
}
