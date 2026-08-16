//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 737/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk737(t1680: f64, t5439: f64, t694: f64, t3993: f64, t2618: f64, t1690: f64, t2861: f64, t1694: f64, t886: f64, t2868: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5441 = t694 * t1680 * t5439;
    let t5443 = 0.21687162600603479684e-1_f64 * t3993;
    let t5444 = 0.10843581300301739842e-1_f64 * t2618;
    let t5445 = t2861 * t1690;
    let t5450 = t886 * t1694;
    let t5455 = -2.0_f64 * t821 - 6.0_f64 * t2868;
    (t5441, t5443, t5444, t5445, t5450, t5455)
}
