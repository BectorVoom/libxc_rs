//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 972/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk972(t1349: f64, t32997: f64, t376: f64, t32737: f64, t32871: f64, t458: f64, t7308: f64, t5775: f64, t32742: f64, t24087: f64, t7309: f64, t32748: f64, t5766: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t138521 = t1349 * t376 * t32997;
    let t138524 = t1349 * t376 * t32737;
    let t138533 = t1349 * t376 * t32871;
    let t138537 = t7308 * t458;
    let t138538 = t138537 * t5775;
    let t138549 = t1349 * t376 * t32742;
    let t138551 = t7309 * t24087;
    let t138557 = t5766 * t32748;
    (t138521, t138524, t138533, t138537, t138538, t138549, t138551, t138557)
}
