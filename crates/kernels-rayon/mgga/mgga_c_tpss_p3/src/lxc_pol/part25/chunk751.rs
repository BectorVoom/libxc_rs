//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 751/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk751(t5036: f64, t981: f64, t1483: f64, t373: f64, t3990: f64, t5013: f64, t5018: f64, t978: f64, t1485: f64, t198: f64, t2814: f64, t330: f64, t4840: f64, t4842: f64, t4846: f64, t4878: f64, t4881: f64, t4947: f64, t4949: f64, t4951: f64, t4955: f64, t4959: f64, t4963: f64, t995: f64) -> (f64, f64, f64, f64) {
    let t5037 = t981 * t5036;
    let t5039 = -2.0_f64 * t1483 * t3990 + t373 * t5013 + 2.0_f64 * t5018 * t978 - t5037 * t978;
    let t5043 = t1485 * t1485;
    let t5047 = -t198 * t2814 * t330 * t5043 + t198 * t330 * t5039 * t995 - t4840 + t4842 - t4846 + t4878 + t4881 + t4947 + t4949 - t4951 + t4955 - t4959 - t4963;
    (t5037, t5039, t5043, t5047)
}
