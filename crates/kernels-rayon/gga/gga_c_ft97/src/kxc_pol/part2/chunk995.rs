//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 995/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk995(t1225: f64, t8232: f64, t15074: f64, t296: f64, t1212: f64, t2739: f64, t2862: f64, t319: f64, t14608: f64, t1248: f64, t840: f64, t871: f64) -> (f64, f64, f64, f64, f64) {
    let t15420 = t8232 * t1225;
    let t15422 = t296 * t15074;
    let t15425 = t1212 * t2739;
    let t15427 = t2862 * t319 * t15425;
    let t15430 = t296 * t14608;
    let t15433 = t1248 * t2739;
    let t15435 = t840 * t871 * t15433;
    (t15420, t15422, t15427, t15430, t15435)
}
