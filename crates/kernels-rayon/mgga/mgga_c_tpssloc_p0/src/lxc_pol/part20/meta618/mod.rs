//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2229;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta618(t40722: f64, t40726: f64, t12858: f64, t2528: f64, t2371: f64, t40729: f64, t40733: f64, t2745: f64, t776: f64, t4205: f64, t9909: f64, t2553: f64, t868: f64, t40736: f64, t10126: f64, t12854: f64, t1877: f64, t2522: f64, t40732: f64, t4119: f64, t4307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46228, t46232, t46235, t46237, t46238, t46239, t46240, t46245, t46252) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2229(t40722, t40726, t12858, t2528, t2371, t40729, t40733, t2745, t776, t4205, t9909, t2553, t868);
        let (t46256, t46257) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2230(t40736, t10126, t12854, t1877, t2522, t2745, t40732, t4119, t4307, t46235, t46237, t46238, t46239, t46240, t46245, t46252);
    (t46228, t46232, t46235, t46237, t46238, t46239, t46245, t46256, t46257)
}
