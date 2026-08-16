//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 600/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk600(t3099: f64, t72: f64, t51: f64, t6: f64, t938: f64, t398: f64, t58: f64, t401: f64, t428: f64, t22591: f64, t379: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25670 = t72 * t3099;
    let t25675 = t938 * t6 * t51;
    let t25676 = t25675 * t398;
    let t25679 = t58 * t938;
    let t25680 = t25679 * t401;
    let t25684 = t25679 * t428;
    let t25685 = t22591 * t25684;
    let t25688 = t930 * t379;
    (t25670, t25676, t25680, t25684, t25685, t25688)
}
