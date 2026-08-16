//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1874;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1875;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta621<F: Float>(t19631: F, t6637: F, t6888: F, t6968: F, t22705: F, t28130: F, t81228: F, t19748: F, t1992: F, t22897: F, t22704: F, t28134: F, t80798: F, t1985: F, t1998: F, t20009: F, t214: F, t1352: F, t26331: F, t6976: F, t97011: F, t1799: F, t90809: F, t26395: F, t5187: F, t22892: F, t22893: F, t28148: F, t19761: F, t1825: F, t22633: F, t90754: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97040, t97043, t97046, t97049) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1874::<F>(t19631, t6637, t6888, t6968, t22705, t28130, t81228, t19748, t1992, t22897, t22704, t28134, t80798);
        let (t97055, t97059, t97063) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1875::<F>(t1985, t1998, t20009, t214, t1352, t26331, t6976, t97011, t1799, t6637, t6888, t90809);
        let (t97067, t97070, t97079, t97083) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1876::<F>(t26395, t5187, t6637, t6888, t22892, t22893, t28148, t19761, t1992, t6976, t1825, t22633, t90754);
    (t97040, t97043, t97046, t97049, t97055, t97059, t97063, t97067, t97070, t97079, t97083)
}
