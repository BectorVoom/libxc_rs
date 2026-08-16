//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2200/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2200(t1774: f64, t26135: f64, t652: f64, t26179: f64, t7461: f64, t25980: f64, t7458: f64, t1983: f64, t28826: f64, t31299: f64, t1388: f64, t6324: f64) -> (f64, f64, f64, f64, f64) {
    let t97865 = 4.0_f64 * t652 * t1774 * t26135;
    let t97869 = 4.0_f64 * t26179 * t7461;
    let t97871 = 4.0_f64 * t7458 * t25980;
    let t97874 = 6.0_f64 * t1983 * t31299 * t28826;
    let t97875 = t6324 * t1388;
    (t97865, t97869, t97871, t97874, t97875)
}
