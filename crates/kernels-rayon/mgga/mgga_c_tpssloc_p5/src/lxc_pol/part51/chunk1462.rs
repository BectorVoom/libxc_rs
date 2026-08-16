//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1462/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1462(t120952: f64, t2039: f64, t102344: f64, t1873: f64, t115241: f64, t122617: f64, t122685: f64, t122718: f64, t122719: f64, t122720: f64, t122721: f64, t122723: f64, t122724: f64, t122725: f64, t122726: f64, t122727: f64, t1458: f64, t26103: f64, t27170: f64, t31532: f64, t33151: f64, t33153: f64, t4072: f64, t6517: f64, t671: f64, t7801: f64, t8445: f64) -> f64 {
    let t122730 = t120952 * t2039;
    let t122731 = t102344 * t1873;
    let t122732 = t115241 * t1458 + t122617 * t671 + t122685 * t1458 + t26103 * t7801 + t27170 * t6517 + t31532 * t4072 + t122718 + t122719 + t122720 + t122721 + t122723 + t122724 + t122725 + t122726 + t122727 + t122730 + t122731 + t33151 + t33153 + t8445;
    t122732
}
