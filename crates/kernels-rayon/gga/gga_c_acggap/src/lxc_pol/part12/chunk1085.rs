//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1085/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1085(t4979: f64, t7561: f64, t4983: f64, t7822: f64, t1181: f64, t21955: f64, t30806: f64, t599: f64, t4987: f64, t7647: f64, t4364: f64, t4963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35309 = t7561 * t4979;
    let t35311 = t7822 * t4983;
    let t35315 = t30806 * t1181 * t599 * t21955;
    let t35317 = t7647 * t4987;
    let t35319 = t7822 * t4364;
    let t35321 = t7561 * t4963;
    (t35309, t35311, t35315, t35317, t35319, t35321)
}
