//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 970/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk970(t21399: f64, t2371: f64, t21446: f64, t89: f64, t9725: f64, t1882: f64, t21450: f64, t21443: f64, t21458: f64, t21454: f64, t1775: f64, t21613: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80748 = t2371 * t21399;
    let t80759 = t89 * t9725 * t21446;
    let t80770 = t1882 * t21450;
    let t80772 = t1882 * t21443;
    let t80819 = t1882 * t21458;
    let t80821 = t1882 * t21454;
    let t80893 = t1775 * t21613;
    (t80748, t80759, t80770, t80772, t80819, t80821, t80893)
}
