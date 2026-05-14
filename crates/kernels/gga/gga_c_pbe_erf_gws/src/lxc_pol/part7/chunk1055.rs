//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1055/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1055<F: Float>(t4408: F, t6638: F, t20597: F, t2118: F, t20085: F, t21064: F, t21068: F, t21106: F, t21115: F, t21118: F, t21123: F, t21127: F, t2272: F, t2305: F, t2312: F, t6207: F, t6276: F, t6637: F, t824: F, t902: F, t905: F) -> (F,) {
    let t21128 = t4408 * t6638;
    let t21132 = t2118 * t20597;
    let t21139 = -t21064 + t21068 + t902 * t905 * t2305 * t20085 / 256.0 + t902 * t905 * t21106 * t824 / 1536.0 + t21115 - 595.0 / 1296.0 * t21118 - t21123 - t21127 + t6637 * t6276 * t21128 / 96.0 + t6637 * t6276 * t21132 / 128.0 - t2312 * t6207 * t2272 / 64.0;
    (t21139,)
}
