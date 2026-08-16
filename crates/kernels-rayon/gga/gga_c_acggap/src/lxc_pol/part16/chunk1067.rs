//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1067/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1067(t1967: f64, t9565: f64, t1410: f64, t525: f64, t1181: f64, t2068: f64, t599: f64, t38647: f64, t157: f64, t1782: f64, t406: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38820 = t1967 * t9565;
    let t38827 = t525 * t1410;
    let t38830 = t2068 * t1181 * t599 * t38827;
    let t38834 = t2068 * t1181 * t599 * t38647;
    let t38837 = t1782 * t406 * t157;
    let t38840 = t2068 * t1181 * t7351 * t38837;
    (t38820, t38827, t38830, t38834, t38837, t38840)
}
