//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta2 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk18;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk19;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk20;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk21;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta2(t34: f64, t36: f64, sigma0: f64, t31: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t38, t39) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk18(t34, t36, sigma0);
        let t40 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk19(t31);
        let (t41, t42, t43) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk20(t40);
        let (t44, t46, t47, t48, t50, t51) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk21(t41, t43, rho1, sigma2);
    (t38, t39, t40, t41, t42, t43, t44, t46, t47, t48, t50, t51)
}
