//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 800/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk800(t15756: f64, t3134: f64, t15742: f64, t3127: f64, t11690: f64, t15737: f64, t15746: f64, t15932: f64, t1787: f64, t15940: f64, t8327: f64, t1587: f64, t3103: f64, t3149: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16418 = t3134 * t15756;
    let t16421 = t3127 * t15742;
    let t16424 = t11690 * t15737;
    let t16427 = t3127 * t15746;
    let t16430 = t1787 * t15932;
    let t16433 = t8327 * t15940;
    let t16439 = t1587 * t3149 * t3103;
    (t16418, t16421, t16424, t16427, t16430, t16433, t16439)
}
