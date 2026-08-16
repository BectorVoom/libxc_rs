//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1201/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1201(t36908: f64, t695: f64, t92016: f64, t2725: f64, t7639: f64, t9194: f64, t26477: f64, t26480: f64, t26474: f64, t26501: f64, t7642: f64, t209: f64, t2155: f64, t8779: f64, t8780: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92022 = t36908 * t695 * t92016;
    let t92025 = t2725 * t9194 * t7639;
    let t92027 = t26480 * t26477;
    let t92029 = t26474 * t92016;
    let t92031 = t7642 * t26501;
    let t92036 = t2155 * t209 * t8779 * t888 * t8780;
    (t92022, t92025, t92027, t92029, t92031, t92036)
}
