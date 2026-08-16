//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 943/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk943(t1460: f64, t672: f64, t925: f64, t140: f64, t3927: f64, t2682: f64, t3941: f64, t8493: f64, t8539: f64, t1465: f64, t242: f64, t8469: f64) -> (f64, f64, f64, f64, f64) {
    let t11640 = t672 * t1460;
    let t11641 = t925 * t11640;
    let t11645 = t140 * t3927;
    let t11647 = t925 * t11645 / 432.0_f64;
    let t11659 = t2682 * t3941 / 432.0_f64;
    let t11661 = t8539 * t8493;
    let t11687 = t242 * t8469 * t1465;
    (t11641, t11647, t11659, t11661, t11687)
}
