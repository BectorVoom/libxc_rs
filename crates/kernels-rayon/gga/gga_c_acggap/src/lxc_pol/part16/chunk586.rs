//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 586/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk586(t1219: f64, t556: f64, t871: f64, t5384: f64, t1662: f64, t814: f64, t467: f64, t495: f64, t3993: f64, t2618: f64, t1690: f64, t2861: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5385 = t1219 * t556;
    let t5386 = t5385 * t871;
    let t5388 = 0.26341796731742046394e1_f64 * t5384 * t5386;
    let t5399 = t1662 * t814;
    let t5439 = t495 * t467;
    let t5443 = 0.21687162600603479684e-1_f64 * t3993;
    let t5444 = 0.10843581300301739842e-1_f64 * t2618;
    let t5445 = t2861 * t1690;
    (t5386, t5388, t5399, t5439, t5443, t5444, t5445)
}
