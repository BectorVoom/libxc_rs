//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 285/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk285(t1787: f64, t2988: f64, t2: f64, t463: f64, t2993: f64, t17: f64, t3050: f64, t9: f64, t18: f64, t464: f64, t458: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3131 = t1787 * t2988;
    let t3134 = t463 * t2;
    let t3135 = t3134 * t2993;
    let t3139 = t9 * t3050 * t17;
    let t3140 = t464 * t18;
    let t3141 = t463 * t3140;
    let t3144 = t458 * t963;
    (t3131, t3135, t3139, t3140, t3141, t3144)
}
