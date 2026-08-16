//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta431(t16558: f64, t31: f64, t65: f64, t5399: f64, t628: f64, t1426: f64, t3961: f64, t3967: f64, t1410: f64, t3997: f64, t1434: f64, t19322: f64, t19323: f64, t19326: f64, t19331: f64, t3962: f64, t5393: f64, t5400: f64, t5403: f64, t642: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19334, t19335, t19338, t19343, t19346, t19349, t19356) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1762(t16558, t31, t65, t5399, t628, t1426, t3961, t3967, t1410, t3997, t1434, t19322, t19323, t19326, t19331, t3962, t5393, t5400, t5403, t642, t80);
    (t19334, t19335, t19338, t19343, t19346, t19349, t19356)
}
