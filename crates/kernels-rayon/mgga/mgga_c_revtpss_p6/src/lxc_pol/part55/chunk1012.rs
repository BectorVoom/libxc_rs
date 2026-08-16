//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1012/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1012(t2163: f64, t4292: f64, t670: f64, t8233: f64, t1519: f64, t1911: f64, t2165: f64, t28183: f64, t28186: f64, t28188: f64, t28190: f64, t28192: f64, t28193: f64, t28201: f64, t28202: f64, t29432: f64, t4248: f64, t4257: f64, t5787: f64, t651: f64, t7586: f64, t7591: f64, t7687: f64) -> (f64, f64, f64) {
    let t29456 = t2163 * t4292;
    let t29459 = t8233 * t670;
    let t29466 = -2.0_f64 * t1519 * t29432 + t1911 * t7687 + t2165 * t5787 - 2.0_f64 * t29456 * t651 - 2.0_f64 * t29459 * t651 - 2.0_f64 * t4248 * t7591 - 2.0_f64 * t4257 * t7586 - t28183 + t28186 - t28188 - t28190 + t28192 - t28193 + t28201 - t28202;
    (t29456, t29459, t29466)
}
