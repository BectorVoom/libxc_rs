//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 787/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk787(t1389: f64, t5843: f64, t28: f64, t1360: f64, t5973: f64, t376: f64, t7345: f64, t1349: f64, t5769: f64, t7309: f64, t7341: f64, t1984: f64, t7339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32691 = t5843 * t1389;
    let t32692 = t28 * t32691;
    let t32695 = t1360 * t5973;
    let t32696 = t28 * t32695;
    let t32699 = t376 * t7345;
    let t32701 = t1349 * t32699 / 9.0_f64;
    let t32703 = t7309 * t5769 / 18.0_f64;
    let t32706 = t376 * t7341;
    let t32708 = t1349 * t32706 / 18.0_f64;
    let t32709 = t1984 * t7339;
    (t32691, t32692, t32695, t32696, t32699, t32701, t32703, t32706, t32708, t32709)
}
