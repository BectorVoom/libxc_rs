//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 968/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk968(t1445: f64, t47225: f64, t833: f64, t47271: f64, t47130: f64, t701: f64, t6066: f64, t7630: f64, t2615: f64, t326: f64, t13871: f64, t1628: f64, t813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47286 = t833 * t1445 * t47225;
    let t47290 = 0.11502877786176224903e2_f64 * t833 * t1445 * t47271;
    let t47294 = t47130 * t701;
    let t47296 = t7630 * t6066 * t47294;
    let t47299 = t2615 * t326 * t47294;
    let t47303 = 0.30674340763136599741e1_f64 * t813 * t1628 * t13871;
    (t47286, t47290, t47294, t47296, t47299, t47303)
}
