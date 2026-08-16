//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 712/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk712(t2033: f64, t4573: f64, t4579: f64, t608: f64, t2040: f64, t612: f64, t77: f64, t1291: f64, t1307: f64, t1314: f64, t4574: f64, t4581: f64, t4584: f64, t4609: f64, t71: f64, t85: f64) -> (f64, f64, f64) {
    let t4614 = t2033 * t4573;
    let t4616 = t608 * t4579;
    let t4618 = t2040 * t4573;
    let t4620 = t612 * t4579;
    let t4622 = 28.0_f64 / 9.0_f64 * t4614 - 4.0_f64 / 3.0_f64 * t4616 + 28.0_f64 / 9.0_f64 * t4618 + 4.0_f64 / 3.0_f64 * t4620;
    let t4623 = t77 * t4622;
    let t4626 = -t4574 * t85 / 12.0_f64 - t4581 * t85 / 12.0_f64 - t4584 * t85 / 6.0_f64 - t1291 * t1314 / 6.0_f64 + t4609 * t85 / 24.0_f64 + t1307 * t1314 / 12.0_f64 + t71 * t4623 / 24.0_f64;
    (t4622, t4623, t4626)
}
