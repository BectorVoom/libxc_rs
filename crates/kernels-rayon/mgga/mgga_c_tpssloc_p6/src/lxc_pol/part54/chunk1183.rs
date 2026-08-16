//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1183/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1183(t31668: f64, t533: f64, t1390: f64, t1983: f64, t1873: f64, t23938: f64, t26977: f64, t6534: f64, t7042: f64, t2039: f64, t31537: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31669 = t533 * t31668;
    let t31670 = t31669 * t1390;
    let t31671 = t1983 * t31670;
    let t31704 = 2.0_f64 * t23938 * t1873;
    let t31706 = 2.0_f64 * t26977 * t1873;
    let t31708 = 2.0_f64 * t7042 * t6534;
    let t31716 = 2.0_f64 * t31537 * t2039;
    let t31717 = t88 * t6534;
    (t31669, t31670, t31671, t31704, t31706, t31708, t31716, t31717)
}
