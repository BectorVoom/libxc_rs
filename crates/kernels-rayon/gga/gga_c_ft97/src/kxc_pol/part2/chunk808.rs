//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 808/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk808(t167: f64, t9132: f64, t12334: f64, t12666: f64, t12670: f64, t12672: f64, t12674: f64, t12676: f64, t12677: f64, t12681: f64, t12685: f64, t12689: f64, t12696: f64, t12700: f64, t1901: f64, t446: f64, t9090: f64, t9097: f64, t9106: f64) -> f64 {
    let t12703 = t9132 * t167;
    let t12704 = t12703 * t12334;
    let t12707 = 2.0_f64 / 3.0_f64 * t446 * t12666 + t12670 + t12672 + t12674 + t12676 + 2.0_f64 / 9.0_f64 * t1901 * t12677 + 2.0_f64 / 9.0_f64 * t1901 * t12681 - t446 * t12685 / 3.0_f64 - t446 * t12689 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t9090 - 2.0_f64 / 27.0_f64 * t9097 + t9106 / 9.0_f64 - t446 * t12696 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t12700 - 4.0_f64 / 9.0_f64 * t1901 * t12704;
    t12707
}
