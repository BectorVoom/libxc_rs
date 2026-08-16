//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 867/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk867(t1675: f64, t3331: f64, t15026: f64, t3623: f64, t1706: f64, t3428: f64, t135: f64, t457: f64, t11529: f64, t1709: f64, t1174: f64, t11588: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15207 = t1675 * t3331;
    let t15245 = t15026 * t3623;
    let t15265 = t1706 * t3428;
    let t15281 = t135 * t457;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15338 = t11588 * t1714;
    (t15207, t15245, t15265, t15281, t15299, t15300, t15338)
}
