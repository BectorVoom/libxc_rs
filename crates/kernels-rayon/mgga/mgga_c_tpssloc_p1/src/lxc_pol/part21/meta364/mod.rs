//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1792;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1793;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta364(t4351: f64, t892: f64, t914: f64, t2837: f64, t4354: f64, t1543: f64, t2841: f64, t2845: f64, t10650: f64, t1557: f64, t2787: f64, t4396: f64, t2770: f64, t3966: f64, t607: f64, t2826: f64, t136: f64, t2250: f64, t4337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13515, t13517, t13519, t13520) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1792(t4351, t892, t914, t2837, t4354, t1543, t2841);
        let (t13522, t13524, t13526, t13528) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1793(t13520, t2845, t10650, t1557, t2787, t4396, t2770, t3966, t607);
        let (t13529, t13530, t13532) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1794(t13528, t2826, t136, t2250, t4337);
    (t13515, t13517, t13519, t13520, t13522, t13524, t13526, t13528, t13529, t13530, t13532)
}
