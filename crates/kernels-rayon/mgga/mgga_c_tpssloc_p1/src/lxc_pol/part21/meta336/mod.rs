//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1714;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1715;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta336(t12619: f64, t72: f64, t1410: f64, t2283: f64, t1426: f64, t2244: f64, t2251: f64, t3997: f64, t608: f64, t1411: f64, t1434: f64, t2245: f64, t2252: f64, t2284: f64, t2304: f64, t3971: f64, t3976: f64, t4018: f64, t629: f64, t642: f64, t66: f64, t80: f64, t1409: f64, t2250: f64, t65: f64, t3966: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12620, t12623, t12630, t12633, t12636, t12645) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1714(t12619, t72, t1410, t2283, t1426, t2244, t2251, t3997, t608, t1411, t1434, t2245, t2252, t2284, t2304, t3971, t3976, t4018, t629, t642, t66, t80);
        let t12648 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1715(t1409, t2250);
        let (t12649, t12652) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1716(t12648, t65, t3966, t607);
    (t12620, t12623, t12630, t12633, t12636, t12645, t12648, t12649, t12652)
}
