//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 972/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk972(t21570: f64, t458: f64, t1775: f64, t21589: f64, t21603: f64, t21585: f64, t21435: f64, t2336: f64, t89: f64, t21420: f64, t681: f64, t21412: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81042 = t458 * t21570;
    let t81048 = t1775 * t21589;
    let t81050 = t1775 * t21603;
    let t81057 = t1775 * t21585;
    let t81095 = t89 * t2336 * t21435;
    let t81102 = t89 * t681 * t21420;
    let t81105 = t89 * t2336 * t21412;
    (t81042, t81048, t81050, t81057, t81095, t81102, t81105)
}
