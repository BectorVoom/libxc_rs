//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1188/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1188(t11741: f64, t11748: f64, t146: f64, t2206: f64, t3177: f64, t3305: f64, t2124: f64, t30049: f64, t3295: f64, t30053: f64, t3308: f64, t5136: f64) -> (f64, f64, f64, f64) {
    let t43561 = t11748 * t11741;
    let t43564 = t146 * t2206 * t3177;
    let t43565 = t43564 * t3305;
    let t43569 = t3295 * t2124 * t30049;
    let t43572 = t5136 * t3308 * t30053;
    (t43561, t43565, t43569, t43572)
}
