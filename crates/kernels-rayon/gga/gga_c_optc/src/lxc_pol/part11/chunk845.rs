//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 845/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk845(t9522: f64, t9530: f64, t16247: f64, t85: f64, t9701: f64, t9703: f64, t9705: f64, t6465: f64, t6477: f64, t6750: f64, t6753: f64, t6771: f64, t6811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16336 = 3.0_f64 * t9522;
    let t16337 = 0.32530742648344572643e-1_f64 * t9530;
    let t16339 = 0.19751789702565206229e-1_f64 * t16247 * t85;
    let t16340 = 60.0_f64 * t9701;
    let t16341 = 36.0_f64 * t9703;
    let t16342 = 96.0_f64 * t9705;
    let t16343 = -t6750 + t6753 + t6465 + t6771 + t16336 + t16337 + t16339 + t6811 + t6477 + t16340 + t16341 + t16342;
    (t16336, t16337, t16339, t16340, t16341, t16342, t16343)
}
