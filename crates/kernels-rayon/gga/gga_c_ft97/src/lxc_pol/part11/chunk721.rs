//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 721/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk721(t2609: f64, t9787: f64, t2360: f64, t761: f64, t2349: f64, t766: f64, t2606: f64, t713: f64, t3885: f64, t2599: f64, t2344: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9788 = t9787 * t2609;
    let t9791 = t761 * t2360;
    let t9792 = t2349 * t766;
    let t9793 = t9791 * t9792;
    let t9794 = t2606 * t9793;
    let t9797 = t2349 * t713;
    let t9798 = t3885 * t9797;
    let t9799 = t2599 * t9798;
    let t9802 = t2344 * t675;
    (t9788, t9792, t9793, t9794, t9797, t9798, t9799, t9802)
}
