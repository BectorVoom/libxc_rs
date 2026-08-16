//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1166/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1166(t13924: f64, t7129: f64, t2508: f64, t39403: f64, t948: f64, t43274: f64, t43275: f64, t43278: f64, t43282: f64, t43283: f64, t43286: f64, t43288: f64, t43289: f64, t43290: f64) -> f64 {
    let t47737 = t7129 * t13924;
    let t47740 = t2508 * t39403 * t948;
    let t47744 = -0.23071578690426672851e-1_f64 * t47737 - 0.23071578690426672851e-1_f64 * t47740 - t43274 + t43275 + 0.76905262301422242837e-2_f64 * t43278 - t43282 - t43283 - t43286 + t43288 - t43289 - 0.42725145723012357132e-3_f64 * t43290;
    t47744
}
