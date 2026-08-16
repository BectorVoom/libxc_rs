//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1640;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1641;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta400(t1653: f64, t3509: f64, t3578: f64, t3516: f64, t1742: f64, t478: f64, t3068: f64, t1244: f64, t11697: f64, t4949: f64, t3577: f64, t3431: f64, t4729: f64, t1174: f64, t1177: f64, t14749: f64, t14753: f64, t14744: f64, t1011: f64, t15031: f64, t1212: f64, t1226: f64, t4965: f64, t11652: f64, t11665: f64, t11678: f64, t11692: f64, t11699: f64, t11703: f64, t1218: f64, t1232: f64, t3496: f64, t3580: f64, t4950: f64, t5002: f64, t4953: f64, t12648: f64, t4972: f64, t4582: f64, t1229: f64, t3242: f64, t14165: f64, t3493: f64, t3508: f64, t4977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15560, t15564, t15569, t15572, t15574, t15578) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1640(t1653, t3509, t3578, t3516, t1742, t478, t3068, t1244, t11697, t4949, t3577, t3431, t4729);
        let (t15590, t15601) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1641(t1174, t15578, t1177, t14749, t14753, t14744, t1011, t15031, t1212, t1226, t4965, t11652, t11665, t11678, t11692, t11699, t11703, t1218, t1232, t15560, t15564, t15569, t15574, t3496, t3580, t4950, t5002);
        let (t15608, t15610, t15612, t15617, t15620, t15621) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1642(t11697, t4953, t3577, t12648, t4972, t4582, t1229, t3242, t14165, t3493, t3508, t4977);
    (t15560, t15564, t15572, t15590, t15601, t15608, t15610, t15612, t15617, t15620, t15621)
}
