//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 663/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk663(t2123: f64, t570: f64, t2367: f64, t321: f64, t118: f64, t25809: f64, t558: f64, t35959: f64, t3839: f64, t3851: f64, t22: f64, t235: f64, t34812: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41063 = t2123 * t570;
    let t41091 = t2367 * t321;
    let t41116 = t118 * t25809;
    let t41122 = t2123 * t558;
    let t41400 = t3839 * t35959;
    let t41407 = t3851 * t35959;
    let t41738 = t235 * t34812 * t22;
    (t41063, t41091, t41116, t41122, t41400, t41407, t41738)
}
