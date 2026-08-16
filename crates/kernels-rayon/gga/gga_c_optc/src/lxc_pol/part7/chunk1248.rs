//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1248/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1248(t24521: f64, t2672: f64, t8101: f64, t889: f64, t2620: f64, t2731: f64, t287: f64, t320: f64, t321: f64, t3695: f64, t2737: f64, t2743: f64) -> (f64, f64, f64, f64, f64) {
    let t25776 = t24521 * t2672;
    let t25781 = t8101 * t889;
    let t25783 = t2731 * t2620;
    let t25788 = 0.85858385084333410912e-1_f64 * t320 * t321 * t3695 * t287;
    let t25789 = t2737 * t2743;
    (t25776, t25781, t25783, t25788, t25789)
}
