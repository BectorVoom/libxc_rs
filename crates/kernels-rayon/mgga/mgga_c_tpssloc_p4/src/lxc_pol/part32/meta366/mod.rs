//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1418;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta366(t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11539: f64, t4724: f64, t1174: f64, t13969: f64, t4983: f64, t3515: f64, t478: f64, t3068: f64, t1244: f64, t11697: f64, t4949: f64, t3577: f64, t3431: f64, t4729: f64, t1011: f64, t15031: f64, t1212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15501, t15503, t15507, t15524, t15548, t15550, t15567) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1418(t1742, t3036, t3503, t3500, t1210, t11539, t4724, t1174, t13969, t4983, t3515, t478);
        let (t15569, t15572, t15574, t15580, t15590, t15591) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1419(t15567, t3068, t1244, t11697, t4949, t3577, t3431, t4729, t1174, t1011, t15031, t1212);
    (t15501, t15503, t15507, t15524, t15548, t15550, t15569, t15572, t15574, t15580, t15590, t15591)
}
