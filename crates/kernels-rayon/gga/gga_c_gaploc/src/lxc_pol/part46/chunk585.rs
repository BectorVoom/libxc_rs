//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 585/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk585(t10205: f64, t471: f64, t3334: f64, t64: f64, t2748: f64, t871: f64, t9097: f64, t9100: f64, t9113: f64, t9115: f64) -> f64 {
    let t10206 = t10205 * t471;
    let t10208 = 4.0_f64 / 3.0_f64 * t3334 * t64;
    let t10209 = t2748 * t871;
    let t10211 = 7.0_f64 / 256.0_f64 * t9097;
    let t10212 = 21.0_f64 / 8192.0_f64 * t9100;
    let t10213 = 7.0_f64 / 8192.0_f64 * t9113;
    let t10214 = 7.0_f64 / 768.0_f64 * t9115;
    let t10215 = t10206 - t10208 + t10209 / 2.0_f64 - t10211 + t10212 - t10213 + t10214;
    t10215
}
