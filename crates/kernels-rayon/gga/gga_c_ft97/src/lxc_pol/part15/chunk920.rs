//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 920/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk920(t21382: f64, t6783: f64, t2378: f64, t37481: f64, t21333: f64, t30815: f64, t4977: f64, t1609: f64, t694: f64, t5005: f64, t695: f64, t1100: f64, t52563: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t66419 = t6783 * t21382;
    let t66422 = t37481 * t2378;
    let t66424 = t30815 * t21333;
    let t66451 = t2378 * t4977;
    let t66482 = t694 * t1609;
    let t66520 = t695 * t5005;
    let t66555 = t1100 * t52563;
    (t66419, t66422, t66424, t66451, t66482, t66520, t66555)
}
