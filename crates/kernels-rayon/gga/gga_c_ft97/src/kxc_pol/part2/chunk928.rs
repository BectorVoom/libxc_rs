//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 928/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk928(t245: f64, t18: f64, t776: f64, t1178: f64, t14366: f64, t1577: f64, t1580: f64, t21: f64, t2624: f64, t267: f64, t363: f64, t4011: f64, t4021: f64, t5: f64, t7742: f64, t920: f64) -> f64 {
    let t246 = 10000000.0_f64 <= t245;
    let t14379 = t776 * t18;
    let t14389 = piecewise3(t246, 0.0_f64, t5 * t14366 * t21 / 4.0_f64 + t5 * t4011 * t363 / 2.0_f64 + t5 * t1178 * t1580 / 4.0_f64 + t5 * t2624 * t920 / 4.0_f64 + t5 * t14379 * t1577 + t5 * t267 * t1577 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t5 * t4021 * t7742);
    t14389
}
