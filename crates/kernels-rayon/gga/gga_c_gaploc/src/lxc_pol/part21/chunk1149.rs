//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1149/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1149(t2464: f64, t2476: f64, t9440: f64, t2349: f64, t4752: f64, t20671: f64, t20688: f64, t20700: f64, t1538: f64, t20539: f64, t21283: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t30927 = t2476 * t2464 * t9440;
    let t30949 = t4752 * t2349;
    let t31018 = 0.17041300423964777634e0_f64 * t20688 * t20671 * t20700;
    let t31021 = t21283 * t1538 * t883 * t20539;
    (t30927, t30949, t31018, t31021)
}
