//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1954;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1955;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1956;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta427(t3403: f64, t4857: f64, t1155: f64, t3395: f64, t4861: f64, t11285: f64, t1694: f64, t3377: f64, t1683: f64, t3333: f64, t11303: f64, t11310: f64, t11415: f64, t15050: f64, t15053: f64, t15056: f64, t15059: f64, t15063: f64, t15066: f64, t15070: f64, t3357: f64, t3401: f64, t4802: f64, t4824: f64, t15139: f64, t15162: f64, t15213: f64, t300: f64, t3411: f64, t4875: f64, t14958: f64, t14963: f64, t14969: f64, t14971: f64, t15038: f64, t15040: f64, t15043: f64, t15046: f64, t15048: f64, t15035: f64, t491: f64, t1246: f64, t15026: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15218, t15219, t15222, t15225, t15226, t15229, t15232) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1954(t3403, t4857, t1155, t3395, t4861, t11285, t1694, t3377, t1683, t3333, t11303, t11310, t11415, t15050, t15053, t15056, t15059, t15063, t15066, t15070, t3357, t3401, t4802, t4824);
        let (t15235, t15237, t15238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1955(t15139, t15162, t15213, t15232, t300, t3411, t4875, t14958, t14963, t14969, t14971, t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15070);
        let t15239 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1956(t15035, t15238);
        let (t15241, t15245) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1957(t15239, t491, t1246, t15026, t3623);
    (t15218, t15219, t15222, t15225, t15226, t15229, t15235, t15237, t15239, t15241, t15245)
}
