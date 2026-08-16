//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1540;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1541;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1542;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1543;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta273(t204: f64, t682: f64, t268: f64, t703: f64, t2419: f64, t2421: f64, t676: f64, t118: f64, t168: f64, t2477: f64, t2510: f64, t725: f64, t740: f64, t9457: f64, t9476: f64, t9484: f64, t9697: f64, t9730: f64, t9734: f64, t9739: f64, t9740: f64, t9752: f64, t9755: f64, t9758: f64, t9759: f64, t9762: f64, t9763: f64, t9766: f64, t9780: f64, t9781: f64, t9789: f64, t2368: f64, t739: f64, t2509: f64, t724: f64, t2406: f64, t2483: f64, t2410: f64, t2415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9790, t9793) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1540(t204, t682, t268, t703);
        let t9797 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1541(t2419, t2421, t268, t676);
        let t9798 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1542(t118, t168, t2477, t2510, t725, t740, t9457, t9476, t9484, t9697, t9730, t9734, t9739, t9740, t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9780, t9781, t9789, t9793, t9797);
        let (t9799, t9803, t9810, t9814, t9820) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1543(t2368, t676, t204, t739, t2509, t724, t2406, t2483, t268);
        let (t9821, t9824) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1544(t2410, t676, t2415, t268);
    (t9790, t9793, t9797, t9798, t9799, t9803, t9810, t9814, t9820, t9821, t9824)
}
