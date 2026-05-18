//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1216/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1216<F: Float>(t1134: F, t13187: F, t1113: F, t13172: F, t13397: F, t2253: F, t2255: F, t2343: F, t343: F, t3747: F, t3781: F, t44372: F, t49239: F, t49279: F, t49281: F, t49283: F, t49285: F, t49295: F, t49299: F, t902: F, t905: F, t9343: F) -> (F, F) {
    let t49305 = t1134 * t13187;
    let t49313 = -t49279 - t49281 - t49283 + t49285 - t2253 * t2255 * t3781 * t13397 * t343 / F::new(128.0) + t49295 + t49299 - F::new(7.0) / F::new(576.0) * t44372 + t902 * t905 * t13172 * t3747 / F::new(512.0) - F::new(5.0) / F::new(32.0) * t2343 * t9343 * t49305 + t902 * t905 * t1113 * t49239 / F::new(1536.0);
    (t49305, t49313)
}
