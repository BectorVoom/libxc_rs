//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 802/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk802<F: Float>(t2164: F, t3168: F, t2206: F, t3191: F, t2133: F, t3039: F, t1114: F, t6187: F, t6566: F, t3116: F, t6605: F, t343: F, t8890: F, t858: F, t2407: F, t2142: F, t3113: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9086 = 7.0 / 144.0 * t2164 * t3168;
    let t9096 = 7.0 / 24.0 * t2206 * t3191;
    let t9108 = t3039 * t2133;
    let t9111 = t1114 * t6187;
    let t9119 = t1114 * t6566;
    let t9123 = 7.0 / 144.0 * t3116 * t6605;
    let t9125 = t8890 * t343;
    let t9126 = t858 * t9125;
    let t9127 = t2407 * t9126;
    let t9142 = 7.0 / 144.0 * t3113 * t2142;
    (t9086, t9096, t9108, t9111, t9119, t9123, t9125, t9127, t9142)
}
