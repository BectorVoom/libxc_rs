//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 415/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk415<F: Float>(t173: F, t934: F, t419: F, t1736: F, t420: F, t2984: F, t1527: F, t2993: F, t18: F, t423: F, t2248: F, t1722: F, t1731: F, t1733: F, t3083: F, t409: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3085 = t173 * t934;
    let t3086 = t419 * t3085;
    let t3088 = t420 * t1736;
    let t3089 = t3088 * t2984;
    let t3090 = t419 * t3089;
    let t3092 = t1527 * t2993;
    let t3093 = t419 * t3092;
    let t3095 = t423 * t18;
    let t3096 = t2248 * t3095;
    let t3097 = t419 * t3096;
    let t3099 = -0.17024962234567901235e-1 * t1722 - t1731 + 0.21281202793209876543e-2 * t1733 - 0.17024962234567901235e-1 * t3083 + 0.21281202793209876543e-2 * t3086 + 0.85124811172839506173e-2 * t3090 - 0.12768721675925925926e-1 * t3093 + 0.12768721675925925926e-1 * t3097;
    let t3100 = t409 * t3099;
    (t3085, t3086, t3088, t3090, t3093, t3095, t3097, t3099, t3100)
}
