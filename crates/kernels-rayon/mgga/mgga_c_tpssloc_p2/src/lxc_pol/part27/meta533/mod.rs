//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1948;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1949;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta533(t25980: f64, t652: f64, t22591: f64, t7687: f64, t1983: f64, t1307: f64, t1845: f64, t8643: f64, t22574: f64, t15868: f64, t2019: f64, t1774: f64, t6534: f64, t2314: f64, t7468: f64, t25965: f64, t25969: f64, t25973: f64, t25975: f64, t25977: f64, t25979: f64, t4028: f64, t4034: f64, t650: f64, t6539: f64, t7472: f64, t7670: f64, t1266: f64, t7467: f64, t6876: f64, t7756: f64, t645: f64, t72: f64, t7431: f64, t1437: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25982, t25985, t25987, t25988, t25989, t25991, t25992, t25993, t25994) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1948(t25980, t652, t22591, t7687, t1983, t1307, t1845, t8643, t22574, t15868, t2019, t1774, t6534);
        let t25999 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1949(t25994, t652, t2314, t7468, t25965, t25969, t25973, t25975, t25977, t25979, t25982, t25987, t25991, t25993, t4028, t4034, t650, t6539, t7472, t7670);
        let (t26002, t26003, t26005, t26006, t26009, t26012) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1950(t4034, t7468, t1266, t7467, t652, t6876, t7756, t645, t72, t7431, t1437, t1864);
    (t25985, t25988, t25989, t25992, t25994, t25999, t26002, t26003, t26005, t26006, t26009, t26012)
}
