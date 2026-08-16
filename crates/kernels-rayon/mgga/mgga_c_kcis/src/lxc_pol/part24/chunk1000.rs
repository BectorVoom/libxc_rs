//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1000/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1000(t26474: f64, t26477: f64, t2398: f64, t2725: f64, t7639: f64, t7636: f64, t7642: f64, t7647: f64, t209: f64, t2746: f64, t7645: f64, t2155: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26478 = t26474 * t26477;
    let t26480 = t2725 * t2398;
    let t26481 = t26480 * t7639;
    let t26483 = t7636 * t26477;
    let t26485 = t7642 * t7647;
    let t26487 = t7642 * t7639;
    let t26490 = t209 * t7645 * t2746;
    let t26491 = t2155 * t26490;
    (t26478, t26480, t26481, t26483, t26485, t26487, t26490, t26491)
}
