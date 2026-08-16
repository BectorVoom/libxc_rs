//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1183/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1183(t2606: f64, t2669: f64, t3835: f64, t7354: f64, t8143: f64, t2269: f64, t2672: f64, t24469: f64, t2574: f64, t2579: f64, t7445: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24521 = t2606 * t2606;
    let t24522 = t2669 * t24521;
    let t24530 = t3835 * t8143 * t7354;
    let t24535 = t2672 * t2269;
    let t24536 = t24535 * t24469;
    let t24540 = t2574 * t2579;
    let t24542 = t854 * t7445;
    (t24521, t24522, t24530, t24536, t24540, t24542)
}
