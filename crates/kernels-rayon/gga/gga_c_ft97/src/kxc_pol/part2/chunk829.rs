//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 829/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk829(t13017: f64, t3439: f64, t2075: f64, t920: f64, t2222: f64, t2221: f64, t2157: f64, t3578: f64, t144: f64, t11593: f64, t13000: f64, t13004: f64, t13007: f64, t13010: f64, t13014: f64, t1901: f64, t446: f64, t9270: f64, t9272: f64, t9274: f64, t9282: f64, t9298: f64, t9300: f64, t9302: f64) -> (f64, f64) {
    let t13018 = t3439 * t13017;
    let t13021 = t920 * t2075;
    let t13022 = t2222 * t13021;
    let t13023 = t2221 * t13022;
    let t13030 = t3578 * t2157;
    let t13031 = t144 * t13030;
    let t13037 = 8.0_f64 / 9.0_f64 * t11593 * t13000 - 8.0_f64 / 27.0_f64 * t11593 * t13004 + 2.0_f64 / 9.0_f64 * t1901 * t13007 - 2.0_f64 / 3.0_f64 * t446 * t13010 + t1901 * t13014 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t13018 + t1901 * t13023 / 9.0_f64 - 8.0_f64 / 27.0_f64 * t9270 - 8.0_f64 / 27.0_f64 * t9272 + t9274 / 9.0_f64 - t9282 / 9.0_f64 - t446 * t13031 / 3.0_f64 - 8.0_f64 / 81.0_f64 * t9298 - 2.0_f64 / 9.0_f64 * t9300 + 2.0_f64 / 81.0_f64 * t9302;
    (t13030, t13037)
}
