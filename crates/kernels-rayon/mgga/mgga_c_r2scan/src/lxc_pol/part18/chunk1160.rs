//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1160/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1160(t35373: f64, t792: f64, t37327: f64, t4176: f64, t10615: f64, t12428: f64, t3275: f64, t11483: f64, t11523: f64, t910: f64, t983: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t42868 = t35373 * t792;
    let t42871 = 15.0_f64 / 8.0_f64 * t37327 * t4176 * t42868;
    let t42874 = 5.0_f64 / 16.0_f64 * t3275 * t10615 * t12428;
    let t42876 = t11523 * t11483 / 2.0_f64;
    let t42877 = t910 * t983;
    let t42878 = t42877 * t481;
    (t42871, t42874, t42876, t42877, t42878)
}
