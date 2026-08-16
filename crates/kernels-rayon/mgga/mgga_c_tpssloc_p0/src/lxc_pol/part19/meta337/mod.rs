//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1202;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta337(t2374: f64, t39516: f64, t9879: f64, t9885: f64, t39325: f64, t39497: f64, t39500: f64, t39506: f64, t9882: f64, t9888: f64, t2430: f64, t9912: f64, t2655: f64, t2745: f64, t2528: f64, t9716: f64, t193: f64, t202: f64, t2752: f64, t39549: f64, t39563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40793, t40795, t40797, t40799, t40801, t40803, t40805, t40807, t40808) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1202(t2374, t39516, t9879, t9885, t39325, t39497, t39500, t39506, t9882, t9888, t2430, t9912);
        let (t40809, t40811, t40818, t40819) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1203(t40808, t2655, t9912, t2745, t2528, t9716, t193, t202, t2752, t39549, t39563, t40793, t40795, t40797, t40799, t40801, t40803, t40805, t40807);
    (t40793, t40795, t40797, t40799, t40801, t40803, t40805, t40807, t40809, t40811, t40818, t40819)
}
