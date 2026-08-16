//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1254/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1254(t54117: f64, t1114: f64, t51266: f64, t1150: f64, t51200: f64, t14028: f64, t3295: f64, t14024: f64, t3113: f64, t3123: f64, t51430: f64, t14538: f64, t51329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54118 = 7.0_f64 / 288.0_f64 * t54117;
    let t54119 = t1114 * t51266;
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54129 = 7.0_f64 / 576.0_f64 * t54128;
    let t54135 = t3113 * t14024;
    let t54136 = 7.0_f64 / 144.0_f64 * t54135;
    let t54152 = t3123 * t51430;
    let t54153 = 7.0_f64 / 144.0_f64 * t54152;
    let t54166 = t14538 * t51329;
    (t54118, t54119, t54126, t54129, t54136, t54153, t54166)
}
