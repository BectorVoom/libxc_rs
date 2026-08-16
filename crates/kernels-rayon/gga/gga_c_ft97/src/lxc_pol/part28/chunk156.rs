//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 156/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk156(t177: f64, t178: f64, t377: f64, t381: f64, t529: f64, t637: f64, t629: f64, t631: f64, t634: f64, t184: f64, t21: f64, t19: f64, t362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t639 = 1.0_f64 / t178 / t177;
    let t641 = 0.14443083333333333333e0_f64 * t377;
    let t643 = 0.234754e0_f64 * t529 - t641 - 0.14443083333333333333e0_f64 * t381;
    let t645 = t637 * t639 * t643;
    let t648 = t629 + t631 * t634 / 6.0_f64 + t631 * t645 / 2.0_f64;
    let t649 = t648 * t184;
    let t650 = t649 * t21;
    let t920 = -t19 - t362;
    (t639, t641, t643, t645, t648, t649, t650, t920)
}
