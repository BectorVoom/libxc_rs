//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta2 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk17;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk18;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk19;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk20;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk21;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk22;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta2(t31: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t40 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk17(t31);
        let (t41, t42, t43) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk18(t40);
        let (t44, t46, t47) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk19(t41, t43, rho1);
        let t48 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk20(t47);
        let (t50, t51) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk21(t46, t48, sigma2);
        let t52 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk22(t31);
    (t40, t41, t42, t43, t44, t46, t47, t48, t50, t51, t52)
}
