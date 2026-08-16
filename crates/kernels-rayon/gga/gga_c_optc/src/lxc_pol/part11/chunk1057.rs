//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1057/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1057(t2910: f64, t27071: f64, t490: f64, t492: f64, t496: f64, t1244: f64, t40: f64, t6524: f64, t108: f64, t1256: f64, t176: f64, t203: f64, t6599: f64) -> (f64, f64, f64, f64) {
    let t28030 = t2910 * t2910;
    let t28031 = 1.0_f64 / t28030;
    let t28109 = 40.0_f64 / 81.0_f64 * t490 * t492 * t27071 * t496;
    let t28141 = t40 * t1244 * t6524;
    let t28145 = t176 * t6599 * t1256 * t108 * t203;
    (t28031, t28109, t28141, t28145)
}
