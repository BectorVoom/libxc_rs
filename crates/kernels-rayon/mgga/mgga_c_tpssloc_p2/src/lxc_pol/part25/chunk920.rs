//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 920/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk920(t1174: f64, t11835: f64, t135: f64, t3551: f64, t3556: f64, t1196: f64, t9258: f64, t974: f64, t1176: f64, t3242: f64, t9288: f64, t11638: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11836 = t1174 * t11835;
    let t11838 = t135 * t3551;
    let t11839 = t1174 * t11838;
    let t11841 = t135 * t3556;
    let t11842 = t1174 * t11841;
    let t11844 = t1196 * t9258;
    let t11845 = t974 * t11844;
    let t11848 = t1176 * t3242;
    let t11849 = t11848 * t9288;
    let t11850 = t974 * t11849;
    let t11853 = t11638 * t475;
    (t11836, t11839, t11842, t11845, t11850, t11853)
}
