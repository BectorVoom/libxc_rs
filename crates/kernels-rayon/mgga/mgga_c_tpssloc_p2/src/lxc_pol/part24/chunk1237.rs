//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1237/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1237(t828: f64, t9632: f64, t2553: f64, t2379: f64, t2631: f64, t776: f64, t1022: f64, t2244: f64, t1068: f64, t3209: f64, t1388: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46519 = t9632 * t828;
    let t46606 = t2553 * t828;
    let t47072 = t2379 * t828;
    let t47320 = t2631 * t776;
    let t49975 = t2244 * t1022;
    let t50775 = t3209 * t1068;
    let t53789 = t1388 * t3734;
    (t46519, t46606, t47072, t47320, t49975, t50775, t53789)
}
