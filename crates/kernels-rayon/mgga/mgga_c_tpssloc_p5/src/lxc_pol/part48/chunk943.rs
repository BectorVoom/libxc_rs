//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 943/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk943(t652: f64, t6534: f64, t7156: f64, t12823: f64, t8533: f64, t31772: f64, t4034: f64, t2018: f64, t26161: f64, t3698: f64, t92169: f64, t31338: f64, t81651: f64, t82074: f64) -> (f64, f64, f64, f64, f64) {
    let t114564 = 4.0_f64 * t652 * t7156 * t6534;
    let t114566 = 2.0_f64 * t12823 * t8533;
    let t114568 = 4.0_f64 * t4034 * t31772;
    let t114573 = 6.0_f64 * t26161 * t92169 * t2018 * t3698;
    let t114592 = t81651 * t82074 * t31338;
    (t114564, t114566, t114568, t114573, t114592)
}
