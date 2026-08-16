//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 888/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk888(t1268: f64, t2200: f64, t2202: f64, t4028: f64, t652: f64, t7458: f64, t7676: f64, t8260: f64, t8274: f64, t8278: f64, t8280: f64) -> f64 {
    let t8283 = 2.0_f64 * t1268 * t8278 + 2.0_f64 * t1268 * t8280 - 2.0_f64 * t2200 * t4028 - 2.0_f64 * t2200 * t7458 + 2.0_f64 * t2202 * t4028 + 2.0_f64 * t2202 * t7676 - 2.0_f64 * t652 * t8260 - 2.0_f64 * t652 * t8274;
    t8283
}
