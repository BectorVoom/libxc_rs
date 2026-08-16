//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1846;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1847;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta517<F: Float>(t25980: F, t652: F, t22591: F, t7687: F, t1983: F, t1307: F, t1845: F, t8643: F, t22574: F, t15868: F, t2019: F, t1774: F, t6534: F, t2314: F, t7468: F, t25965: F, t25969: F, t25973: F, t25975: F, t25977: F, t25979: F, t4028: F, t4034: F, t650: F, t6539: F, t7472: F, t7670: F, t1266: F, t7467: F, t6876: F, t7756: F, t645: F, t72: F, t7431: F, t1437: F, t1864: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25982, t25985, t25987, t25988, t25989, t25991, t25992, t25993, t25994) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1846::<F>(t25980, t652, t22591, t7687, t1983, t1307, t1845, t8643, t22574, t15868, t2019, t1774, t6534);
        let t25999 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1847::<F>(t25994, t652, t2314, t7468, t25965, t25969, t25973, t25975, t25977, t25979, t25982, t25987, t25991, t25993, t4028, t4034, t650, t6539, t7472, t7670);
        let (t26002, t26003, t26005, t26006, t26009, t26012) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1848::<F>(t4034, t7468, t1266, t7467, t652, t6876, t7756, t645, t72, t7431, t1437, t1864);
    (t25985, t25988, t25989, t25992, t25994, t25999, t26002, t26003, t26005, t26006, t26009, t26012)
}
