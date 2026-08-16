//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1328/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1328(t41827: f64, t42110: f64, t42113: f64, t959: f64, t42145: f64, t42148: f64, t42233: f64, t42235: f64, t42238: f64, t42241: f64, t42697: f64, t42699: f64, t42701: f64, t42704: f64, t42708: f64) -> (f64, f64) {
    let t42712 = 0.91082604192152556044e5_f64 * t959 * t42110 * t41827 * t42113;
    let t42713 = t42697 + t42699 - t42701 - t42704 - t42145 + t42148 - t42708 - t42712 - t42233 + t42235 - t42238 - t42241;
    (t42712, t42713)
}
