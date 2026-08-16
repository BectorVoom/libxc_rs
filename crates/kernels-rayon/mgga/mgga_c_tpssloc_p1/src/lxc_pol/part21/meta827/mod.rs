//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta827 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2918;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta827(t14473: f64, t4489: f64, t2906: f64, t42110: f64, t42113: f64, t5774: f64, t959: f64, t10629: f64, t14259: f64, t5790: f64, t10623: f64, t5812: f64, t17951: f64, t2940: f64, t14260: f64, t4483: f64, t2925: f64, t5811: f64, t14480: f64, t10723: f64, t17947: f64, t59637: f64, t60810: f64, t60812: f64, t60814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60816, t60821, t60825, t60827) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2918(t14473, t4489, t2906, t42110, t42113, t5774, t959, t10629, t14259, t5790, t10623, t5812);
        let (t60829, t60831, t60834, t60836, t60839, t60840) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2919(t17951, t2940, t14260, t4483, t2925, t5811, t959, t14480, t10723, t17947, t59637, t60810, t60812, t60814, t60816, t60821, t60825, t60827);
    (t60816, t60821, t60825, t60827, t60829, t60831, t60834, t60836, t60839, t60840)
}
