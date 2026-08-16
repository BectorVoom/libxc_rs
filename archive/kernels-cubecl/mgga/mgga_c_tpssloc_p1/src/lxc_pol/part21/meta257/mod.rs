//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1495;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta257<F: Float>(t3: F, t6470: F, t1401: F, t1458: F, t3941: F, t5371: F, t5456: F, t5493: F, t577: F, t641: F, t71: F, t154: F, t781: F, t202: F, t243: F, t2229: F, t61: F, t119: F, t212: F, t343: F, t984: F, t3034: F, t334: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t6471, t6483, t6509, t6546) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1495::<F>(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t641, t71, t154, t781);
        let (t6589, t6597, t6600, t6733, t6739) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1496::<F>(t202, t243, t2229, t61, t119, t212, t343, t984, t3034, t334);
    (t6471, t6483, t6509, t6546, t6589, t6597, t6600, t6733, t6739)
}
