//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1104/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1104<F: Float>(t11478: F, t13431: F, t3138: F, t3139: F, t1133: F, t13578: F, t2105: F, t2253: F, t2343: F, t2345: F, t3219: F, t3257: F, t343: F, t3803: F, t3854: F, t45974: F, t45990: F, t49908: F, t50049: F, t50051: F, t50056: F, t50073: F, t50077: F, t816: F, t9482: F) -> (F, F) {
    let t50087 = 3.0 / 8.0 * t3138 * t3139 * t11478 * t13431;
    let t50088 = -t50049 + t50051 + t50056 - t2253 * t3257 * t3803 * t816 * t3854 * t343 / 64.0 - t2253 * t9482 * t13578 * t2105 * t1133 * t343 / 48.0 - t50073 + t50077 + t2343 * t2345 * t3219 * t49908 / 96.0 - 7.0 / 32.0 * t45974 + 35.0 / 48.0 * t45990 + t50087;
    (t50087, t50088)
}
