//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 499/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk499(t2402: f64, t2407: f64, t2411: f64, t2415: f64, t2698: f64, t2701: f64, t2946: f64, t898: f64, t900: f64, t2265: f64, t2912: f64, t2913: f64, t2915: f64, t2920: f64, t2925: f64, t2930: f64, t2934: f64, t2941: f64, t631: f64) -> (f64, f64, f64) {
    let t2951 = -0.117377e0_f64 * t2698 + 0.234754e0_f64 * t2701 + t2946 + 0.9628722222222222222e-1_f64 * t2402 - 0.9628722222222222222e-1_f64 * t2407 + 0.28886166666666666666e0_f64 * t2411 - 0.14443083333333333333e0_f64 * t2415;
    let t2953 = t898 * t900 * t2951;
    let t2956 = -t2912 - 2.0_f64 / 9.0_f64 * t2913 - 2.0_f64 / 3.0_f64 * t2915 + t631 * t2920 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t2265 * t2925 - t631 * t2930 / 3.0_f64 + t631 * t2934 / 6.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t2941 + t631 * t2953 / 2.0_f64;
    (t2951, t2953, t2956)
}
