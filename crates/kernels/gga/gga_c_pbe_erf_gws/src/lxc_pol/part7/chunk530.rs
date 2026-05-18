//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 530/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk530<F: Float>(t6: F, t874: F, t2171: F, t2345: F, t2131: F, t2144: F, t2152: F, t2175: F, t2208: F, t2214: F, t2218: F, t2302: F, t2308: F, t2312: F, t2315: F, t2320: F, t2324: F, t2327: F, t2336: F, t2339: F, t2343: F, t902: F, t929: F) -> (F, F, F) {
    let t2346 = t6 * t874;
    let t2348 = t2345 * t2346 * t2171;
    let t2351 = -t2144 - t2152 + F::new(5.0) / F::new(768.0) * t929 * t2302 + t902 * t2308 / F::new(768.0) + t2131 - t2312 * t2315 / F::new(192.0) - F::new(7.0) / F::new(1152.0) * t2320 + F::new(7.0) / F::new(576.0) * t2324 - t929 * t2327 / F::new(768.0) + t2336 + t2208 + t2214 - t2218 + t2175 + t902 * t2339 / F::new(1536.0) + t2343 * t2348 / F::new(192.0);
    (t2346, t2348, t2351)
}
