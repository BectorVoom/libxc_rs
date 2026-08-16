//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1434;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1435;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta479(t11285: f64, t1164: f64, t44154: f64, t78287: f64, t22233: f64, t4869: f64, t21830: f64, t11282: f64, t3403: f64, t18915: f64, t6106: f64, t6270: f64, t1671: f64, t71877: f64, t18686: f64, t6021: f64, t6024: f64, t63755: f64, t21810: f64, t4740: f64, t21813: f64, t51120: f64, t6088: f64, t64537: f64, t19270: f64, t193: f64, t336: f64, t3640: f64, t4700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78310, t78312, t78314, t78318, t78320, t78321) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1434(t11285, t1164, t44154, t78287, t22233, t4869, t21830, t11282, t3403, t18915, t6106, t6270);
        let (t78327, t78329, t78331, t78333, t78335, t78338) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1435(t1671, t71877, t18686, t6021, t6024, t63755, t21810, t4740, t21813, t51120, t1164, t6088, t64537);
        let t78342 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1436(t19270, t193, t336, t3640, t4700, t6270, t78310, t78312, t78314, t78318, t78320, t78321, t78327, t78329, t78331, t78333, t78335, t78338);
    (t78310, t78312, t78314, t78318, t78320, t78327, t78329, t78331, t78333, t78335, t78338, t78342)
}
