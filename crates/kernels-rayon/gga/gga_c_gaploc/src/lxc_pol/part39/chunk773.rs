//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 773/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk773(t13129: f64, t1445: f64, t813: f64, t13019: f64, t833: f64, t13023: f64, t3040: f64, t3267: f64, t10012: f64, t1022: f64, t9438: f64, t2684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13130 = t1445 * t13129;
    let t13132 = 0.46011511144704899612e1_f64 * t813 * t13130;
    let t13133 = t1445 * t13019;
    let t13134 = t833 * t13133;
    let t13136 = t1445 * t13023;
    let t13138 = 0.11502877786176224903e2_f64 * t833 * t13136;
    let t13140 = 0.35750489951850426669e0_f64 * t3267 * t3040;
    let t13141 = t10012 * t1022;
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    (t13130, t13132, t13133, t13134, t13136, t13138, t13140, t13141, t13142, t13143)
}
