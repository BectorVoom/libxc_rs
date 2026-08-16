//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 515/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk515(t2984: f64, t3088: f64, t419: f64, t1527: f64, t2993: f64, t18: f64, t423: f64, t2248: f64, t1722: f64, t1731: f64, t1733: f64, t3083: f64, t3086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3089 = t3088 * t2984;
    let t3090 = t419 * t3089;
    let t3092 = t1527 * t2993;
    let t3093 = t419 * t3092;
    let t3095 = t423 * t18;
    let t3096 = t2248 * t3095;
    let t3097 = t419 * t3096;
    let t3099 = -0.17024962234567901235e-1_f64 * t1722 - t1731 + 0.21281202793209876543e-2_f64 * t1733 - 0.17024962234567901235e-1_f64 * t3083 + 0.21281202793209876543e-2_f64 * t3086 + 0.85124811172839506173e-2_f64 * t3090 - 0.12768721675925925926e-1_f64 * t3093 + 0.12768721675925925926e-1_f64 * t3097;
    (t3089, t3090, t3092, t3093, t3095, t3096, t3097, t3099)
}
