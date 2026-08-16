//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1199/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1199(t10810: f64, t574: f64, t9445: f64, t10868: f64, t6165: f64, t9380: f64, t24063: f64, t30053: f64, t3332: f64, t6535: f64, t9296: f64, t12489: f64, t25169: f64) -> (f64, f64, f64, f64, f64) {
    let t43313 = t574 * t10810 * t9445;
    let t43316 = t6165 * t10868 * t9380;
    let t43319 = t24063 * t3332 * t30053;
    let t43322 = t6535 * t10868 * t9296;
    let t43324 = t25169 * t12489;
    (t43313, t43316, t43319, t43322, t43324)
}
