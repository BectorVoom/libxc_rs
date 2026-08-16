//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2416;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2417;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2418;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta736(t14459: f64, t17954: f64, t959: f64, t17955: f64, t4483: f64, t17304: f64, t17948: f64, t21094: f64, t952: f64, t21238: f64, t2904: f64, t17938: f64, t21101: f64, t2940: f64, t1581: f64, t49541: f64, t68888: f64, t41684: f64, t48688: f64, t48689: f64, t48698: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t41655: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64, t291: f64, t21100: f64, t4497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68934, t68936, t68938, t68940, t68943, t68947, t68949) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2416(t14459, t17954, t959, t17955, t4483, t17304, t17948, t21094, t952, t21238, t2904, t17938);
        let (t68951, t68954, t68972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2417(t21101, t2940, t1581, t49541, t68888, t41684, t48688, t48689, t48698, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t68992 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2418(t41655, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let (t68995, t68998, t68999) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2419(t291, t68972, t68992, t21100, t4497, t959, t68934, t68936, t68938, t68940, t68943, t68947, t68949, t68951, t68954);
    (t68934, t68936, t68938, t68940, t68943, t68947, t68949, t68951, t68954, t68995, t68998, t68999)
}
