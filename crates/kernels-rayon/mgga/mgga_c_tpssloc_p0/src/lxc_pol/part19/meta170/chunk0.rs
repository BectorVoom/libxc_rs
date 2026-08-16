//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 801/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk801(t816: f64, t9612: f64, t2553: f64, t776: f64, t2701: f64, t820: f64, t120: f64, t2678: f64) -> (f64, f64, f64, f64) {
    let t9613 = t9612 * t816;
    let t9616 = t776 * t2553;
    let t9618 = t2701 * t820 * t9616;
    let t9621 = t120 * t2678;
    (t9613, t9616, t9618, t9621)
}
