//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1382/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1382(t3957: f64, t6884: f64, t124: f64, t21969: f64, t800: f64, t6850: f64, t9744: f64, t125: f64, t6861: f64, t3936: f64, t9835: f64, t1414: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22038 = t3957 * t6884;
    let t22040 = t124 * t21969;
    let t22041 = t800 * t22040;
    let t22044 = t9744 * t6850;
    let t22046 = t125 * t6861;
    let t22048 = t3936 * t22046 * t9835;
    let t22052 = t1414 * t828 * t21969;
    (t22038, t22041, t22044, t22046, t22048, t22052)
}
