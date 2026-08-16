//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1199/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1199(t10643: f64, t7137: f64, t2508: f64, t7226: f64, t7291: f64, t8483: f64, t21446: f64, t3009: f64, t21783: f64, t27837: f64, t27840: f64, t27844: f64, t27848: f64, t27853: f64, t27856: f64, t27858: f64, t27860: f64, t471: f64) -> (f64, f64, f64, f64, f64) {
    let t32277 = 0.14355648962932151996e0_f64 * t7137 * t10643;
    let t32281 = 0.92286314761706691402e-1_f64 * t2508 * t7226 * t8483 * t7291;
    let t32285 = 0.92286314761706691402e-1_f64 * t2508 * t7226 * t3009 * t21446;
    let t32289 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t3009 * t21783;
    let t32300 = (189.0_f64 / 512.0_f64 * t27837 - 2499.0_f64 / 16384.0_f64 * t27840 + 1239.0_f64 / 524288.0_f64 * t27844 - 441.0_f64 / 0.16777216e8_f64 * t27848 + 147.0_f64 / 0.16777216e8_f64 * t27853 - 413.0_f64 / 524288.0_f64 * t27856 + 833.0_f64 / 16384.0_f64 * t27858 - 63.0_f64 / 512.0_f64 * t27860) * t471;
    (t32277, t32281, t32285, t32289, t32300)
}
