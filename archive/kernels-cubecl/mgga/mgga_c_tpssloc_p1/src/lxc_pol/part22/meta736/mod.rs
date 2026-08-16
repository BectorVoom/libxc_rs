//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2416;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2417;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2418;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta736<F: Float>(t14459: F, t17954: F, t959: F, t17955: F, t4483: F, t17304: F, t17948: F, t21094: F, t952: F, t21238: F, t2904: F, t17938: F, t21101: F, t2940: F, t1581: F, t49541: F, t68888: F, t41684: F, t48688: F, t48689: F, t48698: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F, t41655: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F, t291: F, t21100: F, t4497: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68934, t68936, t68938, t68940, t68943, t68947, t68949) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2416::<F>(t14459, t17954, t959, t17955, t4483, t17304, t17948, t21094, t952, t21238, t2904, t17938);
        let (t68951, t68954, t68972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2417::<F>(t21101, t2940, t1581, t49541, t68888, t41684, t48688, t48689, t48698, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t68992 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2418::<F>(t41655, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let (t68995, t68998, t68999) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2419::<F>(t291, t68972, t68992, t21100, t4497, t959, t68934, t68936, t68938, t68940, t68943, t68947, t68949, t68951, t68954);
    (t68934, t68936, t68938, t68940, t68943, t68947, t68949, t68951, t68954, t68995, t68998, t68999)
}
