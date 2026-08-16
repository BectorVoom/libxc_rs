//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1074/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1074(t2161: f64, t5148: f64, t37638: f64, t2111: f64, t6461: f64, t6072: f64, t20421: f64, t6162: f64, t6327: f64, t10833: f64, t776: f64, t1615: f64, t269: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38149 = t2161 * t5148;
    let t38150 = t38149 * t37638;
    let t38152 = t2111 * t6461;
    let t38153 = t38152 * t6072;
    let t38164 = t6327 * t20421 * t6162;
    let t38165 = 0.25705033881751801528e-4_f64 * t38164;
    let t38166 = t776 * t10833;
    let t38168 = t1615 * t269;
    (t38149, t38150, t38152, t38153, t38165, t38166, t38168)
}
