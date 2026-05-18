//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1374/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1374<F: Float>(t14185: F, t15483: F, t15543: F, t2376: F, t2408: F, t2409: F, t335: F, t338: F, t3717: F, t4110: F, t53915: F, t55660: F, t55672: F, t57311: F, t57319: F, t57324: F, t57326: F, t57330: F, t57332: F, t57334: F, t57338: F, t6781: F, t892: F, t9283: F, t9926: F) -> F {
    let t58516 = -t335 * t338 * t892 * t15483 / F::new(96.0) - t55660 + F::new(5.0) / F::new(384.0) * t57311 + t57319 / F::new(1536.0) - t2408 * t9283 * t14185 * t9926 / F::new(12.0) + t57324 / F::new(384.0) - F::new(7.0) / F::new(72.0) * t57326 + t55672 + t57330 / F::new(384.0) + t57332 / F::new(12.0) + t57334 / F::new(4.0) - t53915 - F::new(7.0) / F::new(288.0) * t57338 + t2408 * t2409 * t6781 * t15543 / F::new(48.0) + t2408 * t2409 * t2376 * t4110 * t3717 / F::new(48.0);
    t58516
}
