//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1120/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1120(t47130: f64, t701: f64, t6066: f64, t7630: f64, t2615: f64, t326: f64, t13871: f64, t1628: f64, t813: f64, t13875: f64, t833: f64, t13879: f64, t317: f64, t769: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47294 = t47130 * t701;
    let t47296 = t7630 * t6066 * t47294;
    let t47299 = t2615 * t326 * t47294;
    let t47303 = 0.30674340763136599741e1_f64 * t813 * t1628 * t13871;
    let t47306 = 0.30674340763136599741e1_f64 * t833 * t1628 * t13875;
    let t47309 = 0.35750489951850426669e0_f64 * t769 * t13879 * t317;
    (t47294, t47296, t47299, t47303, t47306, t47309)
}
