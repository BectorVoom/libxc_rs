//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 860/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk860(t1137: f64, t1403: f64, t1427: f64, t247: f64, t33499: f64, t33573: f64, t33589: f64, t33592: f64, t33594: f64, t35547: f64, t35550: f64, t35573: f64, t35605: f64, t35640: f64, t35679: f64, t35693: f64, t35706: f64, t35729: f64, t35738: f64, t35744: f64, t35753: f64, t35779: f64, t6749: f64, t7558: f64) -> f64 {
    let t35785 = -t33499 * t6749 / 18.0_f64 + 2.0_f64 * t35547 - t1403 * t35550 / 3.0_f64 - t247 * t35744 + 4.0_f64 * t35729 - 12.0_f64 * t35605 + 8.0_f64 * t35738 + 8.0_f64 * t35573 + 4.0_f64 * t35640 + t1403 * t35753 / 6.0_f64 - t1137 * t7558 - t33573 - t33589 + t33592 - t33594 + t35779 * t1427 / 6.0_f64 - 2.0_f64 * t35693 - 2.0_f64 * t35679 - 2.0_f64 * t35706;
    t35785
}
