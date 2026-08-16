//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1302/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1302(t1306: f64, t31186: f64, t31188: f64, t31190: f64, t31196: f64, t31198: f64, t31599: f64, t31604: f64, t31608: f64, t31610: f64, t31612: f64, t3282: f64, t9759: f64) -> f64 {
    let t31616 = -3.0_f64 * t1306 * t3282 * t9759 + t31186 + t31188 + t31190 - t31196 + t31198 + t31599 - t31604 - t31608 + t31610 - t31612;
    t31616
}
