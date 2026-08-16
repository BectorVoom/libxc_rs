//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1640;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1641;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta400<F: Float>(t1653: F, t3509: F, t3578: F, t3516: F, t1742: F, t478: F, t3068: F, t1244: F, t11697: F, t4949: F, t3577: F, t3431: F, t4729: F, t1174: F, t1177: F, t14749: F, t14753: F, t14744: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F, t11652: F, t11665: F, t11678: F, t11692: F, t11699: F, t11703: F, t1218: F, t1232: F, t3496: F, t3580: F, t4950: F, t5002: F, t4953: F, t12648: F, t4972: F, t4582: F, t1229: F, t3242: F, t14165: F, t3493: F, t3508: F, t4977: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15560, t15564, t15569, t15572, t15574, t15578) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1640::<F>(t1653, t3509, t3578, t3516, t1742, t478, t3068, t1244, t11697, t4949, t3577, t3431, t4729);
        let (t15590, t15601) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1641::<F>(t1174, t15578, t1177, t14749, t14753, t14744, t1011, t15031, t1212, t1226, t4965, t11652, t11665, t11678, t11692, t11699, t11703, t1218, t1232, t15560, t15564, t15569, t15574, t3496, t3580, t4950, t5002);
        let (t15608, t15610, t15612, t15617, t15620, t15621) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1642::<F>(t11697, t4953, t3577, t12648, t4972, t4582, t1229, t3242, t14165, t3493, t3508, t4977);
    (t15560, t15564, t15572, t15590, t15601, t15608, t15610, t15612, t15617, t15620, t15621)
}
