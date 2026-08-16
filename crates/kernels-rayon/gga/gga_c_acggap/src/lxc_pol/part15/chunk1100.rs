//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1100/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1100(t31428: f64, t9614: f64, t2030: f64, t20559: f64, t8923: f64, t1016: f64, t2060: f64, t507: f64, t8928: f64, t301: f64, t4256: f64, t7450: f64, t9536: f64) -> (f64, f64, f64, f64) {
    let t39002 = t31428 * t9614;
    let t39005 = t2030 * t20559 * t8923;
    let t39009 = t2060 * t507 * t1016 * t8928;
    let t39013 = t7450 * t4256 * t9536 * t301;
    (t39002, t39005, t39009, t39013)
}
