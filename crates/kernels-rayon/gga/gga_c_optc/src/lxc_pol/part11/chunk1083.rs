//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1083/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1083(t4620: f64, t6956: f64, t22889: f64, t4616: f64, t4727: f64, t6766: f64, t1310: f64, t9430: f64, t133: f64, t193: f64, t197: f64, t4599: f64) -> (f64, f64, f64, f64, f64) {
    let t38770 = t6956 * t4620;
    let t38783 = t22889 * t4616;
    let t38910 = t4727 * t6766;
    let t38936 = t1310 * t9430;
    let t39007 = t193 * t133 * t4599 * t197;
    (t38770, t38783, t38910, t38936, t39007)
}
