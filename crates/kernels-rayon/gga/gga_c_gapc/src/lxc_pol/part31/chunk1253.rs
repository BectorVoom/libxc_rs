//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1253/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1253(t11438: f64, t21649: f64, t3021: f64, t1649: f64, t33303: f64, t5553: f64, t27149: f64, t520: f64, t9061: f64, t11449: f64, t11451: f64, t1803: f64, t190: f64, t21183: f64) -> (f64, f64, f64, f64, f64) {
    let t34791 = t11438 * t3021 * t21649;
    let t34793 = t33303 * t1649;
    let t34794 = t5553 * t34793;
    let t34797 = t9061 * t520 * t27149;
    let t34802 = t1803 * t190 * t11449 * t11451 * t21183;
    (t34791, t34793, t34794, t34797, t34802)
}
