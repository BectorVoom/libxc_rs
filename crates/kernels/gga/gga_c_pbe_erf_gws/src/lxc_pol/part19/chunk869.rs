//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 869/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk869<F: Float>(t3028: F, t369: F, t1109: F, t931: F, t2164: F, t3168: F, t2206: F, t3191: F, t2133: F, t3039: F, t1114: F, t6187: F) -> (F, F, F, F, F, F) {
    let t9053 = t3028 * t369;
    let t9056 = t1109 * t931;
    let t9086 = F::new(7.0) / F::new(144.0) * t2164 * t3168;
    let t9096 = F::new(7.0) / F::new(24.0) * t2206 * t3191;
    let t9108 = t3039 * t2133;
    let t9111 = t1114 * t6187;
    (t9053, t9056, t9086, t9096, t9108, t9111)
}
