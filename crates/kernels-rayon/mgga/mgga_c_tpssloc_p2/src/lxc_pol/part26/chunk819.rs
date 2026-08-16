//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 819/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk819(t2701: f64, t820: f64, t9616: f64, t120: f64, t2678: f64, t4180: f64, t829: f64, t2631: f64, t2632: f64, t776: f64, t2645: f64, t2646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9618 = t2701 * t820 * t9616;
    let t9621 = t120 * t2678;
    let t9623 = t4180 * t9621 * t829;
    let t9626 = t120 * t2631;
    let t9627 = t2632 * t776;
    let t9629 = t2645 * t9626 * t9627;
    let t9632 = t2632 * t2678;
    let t9634 = t4180 * t2646 * t9632;
    (t9618, t9621, t9623, t9626, t9627, t9629, t9632, t9634)
}
