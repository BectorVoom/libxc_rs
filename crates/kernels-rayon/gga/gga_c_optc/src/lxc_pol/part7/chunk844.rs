//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 844/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk844(t7920: f64, t914: f64, t7925: f64, t2731: f64, t889: f64, t155: f64, t329: f64, t7312: f64, t2620: f64, t947: f64, t331: f64, t7895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8092 = t914 * t7920;
    let t8095 = t914 * t7925;
    let t8098 = t2731 * t889;
    let t8101 = t155 * t329 * t7312;
    let t8104 = t947 * t2620;
    let t8107 = 0.22391424203717421017e-2_f64 * t331 * t7895;
    (t8092, t8095, t8098, t8101, t8104, t8107)
}
