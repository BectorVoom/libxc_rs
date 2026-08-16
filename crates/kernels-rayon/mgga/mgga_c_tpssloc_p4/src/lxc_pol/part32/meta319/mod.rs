//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta319(t3355: f64, t432: f64, t427: f64, t1094: f64, t3263: f64, t11135: f64, t11203: f64, t1176: f64, t698: f64, t1179: f64, t1174: f64, t135: f64, t3439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11420, t11424, t11444, t11459, t11487, t11529, t11531, t11539) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1347(t3355, t432, t427, t1094, t3263, t11135, t11203, t1176, t698, t1179, t1174, t135, t3439);
    (t11420, t11424, t11444, t11459, t11487, t11529, t11531, t11539)
}
