//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 977/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk977(t1445: f64, t47322: f64, t807: f64, t41411: f64, t47130: f64, t7290: f64, t4820: f64, t7513: f64, t13892: f64, t5676: f64, t12161: f64, t2033: f64, t2365: f64, t2610: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47462 = 0.23005755572352449806e1_f64 * t807 * t1445 * t47322;
    let t47463 = 0.51123901271894332903e0_f64 * t41411;
    let t47484 = t7290 * t47130;
    let t47486 = t7513 * t4820 * t47484;
    let t47488 = t5676 * t13892;
    let t47492 = t2033 * t2365 * t2610 * t12161;
    (t47462, t47463, t47484, t47486, t47488, t47492)
}
