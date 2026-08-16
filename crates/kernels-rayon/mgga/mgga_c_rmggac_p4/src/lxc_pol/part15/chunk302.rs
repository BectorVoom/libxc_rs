//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 302/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk302(t181: f64, t1811: f64, t1373: f64, t1416: f64, t1417: f64, t1419: f64, t618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1813 = 0.19751673498613801407e-1_f64 * t1811 * t181;
    let t1814 = 0.11696447245269292414e1_f64 * t1373;
    let t1815 = 2.0_f64 * t1416;
    let t1816 = 8.0_f64 * t1417;
    let t1817 = 8.0_f64 * t1419;
    let t1818 = t618 * t618;
    (t1813, t1814, t1815, t1816, t1817, t1818)
}
