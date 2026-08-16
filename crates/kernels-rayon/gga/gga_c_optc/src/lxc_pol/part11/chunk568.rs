//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 568/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk568(t4635: f64, t4636: f64, t124: f64, t4599: f64, t4595: f64, t121: f64, t1268: f64, t2060: f64, t3406: f64, t641: f64) -> (f64, f64, f64, f64) {
    let t4637 = t4635 + t4636;
    let t4643 = t124 * t4599;
    let t4646 = t124 * t4595;
    let t4649 = -0.12897460341341234505e3_f64 * t4637 * t121 * t124 + 0.7738476204804740703e3_f64 * t3406 * t1268 - 0.15476952409609481406e4_f64 * t2060 * t4643 + 0.38692381024023703515e3_f64 * t641 * t4646;
    (t4637, t4643, t4646, t4649)
}
