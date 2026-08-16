//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta410 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1912;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1913;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1914;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1915;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1916;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1917;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1918;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta410(t14736: f64, t3240: f64, t123: f64, t2250: f64, t4723: f64, t2244: f64, t1088: f64, t3247: f64, t3966: f64, t607: f64, t4728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14737, t14738) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1912(t14736, t3240, t123);
        let t14740 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1913(t2250, t4723);
        let (t14741, t14742) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1914(t14740, t3240, t123);
        let t14744 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1915(t2244, t4723);
        let (t14745, t14746) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1916(t1088, t14744, t123);
        let t14749 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1917(t3247, t3966, t607);
        let (t14750, t14751) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1918(t1088, t14749, t123);
        let t14753 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1919(t2250, t4728);
    (t14737, t14738, t14740, t14741, t14742, t14744, t14745, t14746, t14749, t14750, t14751, t14753)
}
