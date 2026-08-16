//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1220/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1220(t27837: f64, t27840: f64, t27844: f64, t27848: f64, t27853: f64, t27856: f64, t27858: f64, t27860: f64, t471: f64, t10657: f64, t64: f64, t3427: f64, t90: f64) -> (f64, f64, f64) {
    let t32300 = (189.0_f64 / 512.0_f64 * t27837 - 2499.0_f64 / 16384.0_f64 * t27840 + 1239.0_f64 / 524288.0_f64 * t27844 - 441.0_f64 / 0.16777216e8_f64 * t27848 + 147.0_f64 / 0.16777216e8_f64 * t27853 - 413.0_f64 / 524288.0_f64 * t27856 + 833.0_f64 / 16384.0_f64 * t27858 - 63.0_f64 / 512.0_f64 * t27860) * t471;
    let t32302 = 8.0_f64 / 3.0_f64 * t10657 * t64;
    let t32304 = 4.0_f64 / 3.0_f64 * t3427 * t90;
    (t32300, t32302, t32304)
}
