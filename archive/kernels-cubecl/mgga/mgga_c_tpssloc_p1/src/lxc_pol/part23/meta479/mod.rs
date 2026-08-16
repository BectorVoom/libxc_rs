//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1434;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1435;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta479<F: Float>(t11285: F, t1164: F, t44154: F, t78287: F, t22233: F, t4869: F, t21830: F, t11282: F, t3403: F, t18915: F, t6106: F, t6270: F, t1671: F, t71877: F, t18686: F, t6021: F, t6024: F, t63755: F, t21810: F, t4740: F, t21813: F, t51120: F, t6088: F, t64537: F, t19270: F, t193: F, t336: F, t3640: F, t4700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t78310, t78312, t78314, t78318, t78320, t78321) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1434::<F>(t11285, t1164, t44154, t78287, t22233, t4869, t21830, t11282, t3403, t18915, t6106, t6270);
        let (t78327, t78329, t78331, t78333, t78335, t78338) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1435::<F>(t1671, t71877, t18686, t6021, t6024, t63755, t21810, t4740, t21813, t51120, t1164, t6088, t64537);
        let t78342 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1436::<F>(t19270, t193, t336, t3640, t4700, t6270, t78310, t78312, t78314, t78318, t78320, t78321, t78327, t78329, t78331, t78333, t78335, t78338);
    (t78310, t78312, t78314, t78318, t78320, t78327, t78329, t78331, t78333, t78335, t78338, t78342)
}
