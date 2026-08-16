//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1204/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1204(t43538: f64, t71276: f64, t71298: f64, t71305: f64, t71319: f64, t83728: f64, t83770: f64, t83772: f64, t83781: f64, t83789: f64, t83792: f64, t90326: f64, t90330: f64, t90335: f64, t90468: f64) -> f64 {
    let t91080 = 40.0_f64 / 243.0_f64 * t83728 - 5.0_f64 / 16.0_f64 * t90326 - t90330 / 4.0_f64 + 16.0_f64 / 27.0_f64 * t71276 + t43538 + 8.0_f64 / 3.0_f64 * t90335 + t90468 / 6.0_f64 + 4.0_f64 / 9.0_f64 * t83770 - 8.0_f64 / 27.0_f64 * t83772 + 8.0_f64 / 9.0_f64 * t83781 - 8.0_f64 / 9.0_f64 * t83789 + 8.0_f64 / 3.0_f64 * t83792 - 16.0_f64 / 81.0_f64 * t71298 + 16.0_f64 / 27.0_f64 * t71305 - 8.0_f64 / 27.0_f64 * t71319;
    t91080
}
