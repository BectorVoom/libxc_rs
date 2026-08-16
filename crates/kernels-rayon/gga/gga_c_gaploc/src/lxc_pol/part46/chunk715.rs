//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 715/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk715(t12709: f64, t10677: f64, t935: f64, t1445: f64, t813: f64, t2949: f64, t3234: f64, t13019: f64, t833: f64, t13023: f64, t3040: f64, t3267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13124 = 0.19171462976960374838e1_f64 * t12709;
    let t13125 = t10677 * t935;
    let t13126 = t1445 * t13125;
    let t13127 = t813 * t13126;
    let t13129 = t2949 * t3234;
    let t13130 = t1445 * t13129;
    let t13132 = 0.46011511144704899612e1_f64 * t813 * t13130;
    let t13133 = t1445 * t13019;
    let t13134 = t833 * t13133;
    let t13136 = t1445 * t13023;
    let t13138 = 0.11502877786176224903e2_f64 * t833 * t13136;
    let t13140 = 0.35750489951850426669e0_f64 * t3267 * t3040;
    (t13124, t13125, t13126, t13127, t13129, t13130, t13132, t13133, t13134, t13136, t13138, t13140)
}
