//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk638;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk639;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk640;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk641;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk642;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk643;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk644;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta89(t2229: f64, t19: f64, t2218: f64, t2220: f64, t2222: f64, t2224: f64, t2226: f64, t2228: f64, t601: f64, t604: f64, t84: f64, t85: f64, t24: f64, t645: f64, t607: f64, t65: f64, t11: f64, t2219: f64, t25: f64, t28: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2230, t2232, t2233, t2235) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk638(t2229, t19, t2218, t2220, t2222, t2224, t2226, t2228, t601, t604);
        let t2239 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk639(t84, t85);
        let t2240 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk640(t2239, t24);
        let t2241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk641(t645);
        let t2244 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk642(t607);
        let (t2245, t2248) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk643(t2244, t65, t11, t2219);
        let t2249 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk644(t2248);
        let t2250 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk645(t25, t28, t2249, zeta_threshold);
    (t2230, t2232, t2233, t2235, t2239, t2240, t2241, t2244, t2245, t2248, t2249, t2250)
}
