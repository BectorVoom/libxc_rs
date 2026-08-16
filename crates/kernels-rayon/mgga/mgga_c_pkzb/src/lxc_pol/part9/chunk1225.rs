//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1225/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1225(t1306: f64, t21027: f64, t21030: f64, t21033: f64, t21037: f64, t21039: f64, t21291: f64, t21299: f64, t21301: f64, t21306: f64, t21308: f64, t2153: f64, t2993: f64, t6065: f64) -> f64 {
    let t21309 = 6.0_f64 * t1306 * t2153 * t2993 * t6065 - t21027 - t21030 - t21033 + t21037 + t21039 + t21291 - t21299 + t21301 - t21306 - t21308;
    t21309
}
