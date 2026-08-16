//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1078/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1078(t13257: f64, t812: f64, t242: f64, t9972: f64, t820: f64, t9645: f64, t4290: f64, t808: f64, t68: f64, t9971: f64, t226: f64, t4280: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13258 = t812 * t13257;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13350 = t9645 * t820;
    let t13390 = t808 * t4290;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13453 = t808 * t4280;
    (t13258, t13262, t13350, t13390, t13397, t13453)
}
