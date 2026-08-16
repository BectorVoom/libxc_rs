//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1044/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1044(t486: f64, t548: f64, t4149: f64, t874: f64, t1305: f64, t2293: f64, t475: f64, t588: f64, t61: f64) -> (f64, f64, f64, f64, f64) {
    let t20019 = t548 * t486;
    let t20065 = t4149 * t874;
    let t20073 = t874 * t1305;
    let t20117 = t2293 * t475;
    let t20157 = t61 * t588;
    (t20019, t20065, t20073, t20117, t20157)
}
