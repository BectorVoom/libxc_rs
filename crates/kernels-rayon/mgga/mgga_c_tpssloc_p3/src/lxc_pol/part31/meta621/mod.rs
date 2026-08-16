//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1874;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1875;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta621(t19631: f64, t6637: f64, t6888: f64, t6968: f64, t22705: f64, t28130: f64, t81228: f64, t19748: f64, t1992: f64, t22897: f64, t22704: f64, t28134: f64, t80798: f64, t1985: f64, t1998: f64, t20009: f64, t214: f64, t1352: f64, t26331: f64, t6976: f64, t97011: f64, t1799: f64, t90809: f64, t26395: f64, t5187: f64, t22892: f64, t22893: f64, t28148: f64, t19761: f64, t1825: f64, t22633: f64, t90754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97040, t97043, t97046, t97049) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1874(t19631, t6637, t6888, t6968, t22705, t28130, t81228, t19748, t1992, t22897, t22704, t28134, t80798);
        let (t97055, t97059, t97063) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1875(t1985, t1998, t20009, t214, t1352, t26331, t6976, t97011, t1799, t6637, t6888, t90809);
        let (t97067, t97070, t97079, t97083) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1876(t26395, t5187, t6637, t6888, t22892, t22893, t28148, t19761, t1992, t6976, t1825, t22633, t90754);
    (t97040, t97043, t97046, t97049, t97055, t97059, t97063, t97067, t97070, t97079, t97083)
}
