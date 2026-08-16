//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 814/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk814(t113: f64, t7194: f64, t2530: f64, t494: f64, t1550: f64, t920: f64, t1553: f64, t3270: f64, t792: f64, t1561: f64, t983: f64, t2847: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7195 = t7194 * t113;
    let t7197 = t2530 * t494;
    let t7202 = t920 * t1550;
    let t7204 = t920 * t1553;
    let t7206 = t3270 * t792;
    let t7217 = t1561 * t983;
    let t7218 = t7217 * t792;
    let t7221 = t498 * t2847;
    (t7195, t7197, t7202, t7204, t7206, t7218, t7221)
}
