//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 396/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk396(t1766: f64, t965: f64, t1775: f64, t959: f64, t1780: f64, t2: f64, t463: f64, t17: f64, t3050: f64, t9: f64, t458: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3119 = t1766 * t965;
    let t3125 = t1775 * t959;
    let t3127 = t1780 * t2;
    let t3134 = t463 * t2;
    let t3139 = t9 * t3050 * t17;
    let t3144 = t458 * t963;
    (t3119, t3125, t3127, t3134, t3139, t3144)
}
