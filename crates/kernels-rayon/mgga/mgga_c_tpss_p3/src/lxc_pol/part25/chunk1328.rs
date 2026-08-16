//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1328/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1328(t18246: f64, t69863: f64, t1006: f64, t4802: f64, t64879: f64, t70243: f64, t4701: f64, t1497: f64, t3683: f64, t823: f64, t21262: f64, t61703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70813 = t18246 * t69863;
    let t70816 = t1006 * t4802;
    let t70828 = t64879 * t70243;
    let t70839 = t1006 * t4701;
    let t70844 = t823 * t1497 * t3683;
    let t70847 = t61703 * t21262;
    (t70813, t70816, t70828, t70839, t70844, t70847)
}
