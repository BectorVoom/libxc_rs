//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2397;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2399;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta731<F: Float>(t13623: F, t5705: F, t17271: F, t4378: F, t21180: F, t2798: F, t896: F, t2815: F, t136: F, t68569: F, t908: F, t41684: F, t48946: F, t48947: F, t48956: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F, t41904: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F, t894: F, t901: F, t60308: F, t60310: F, t60312: F, t68457: F, t68496: F, t68532: F, t68565: F, t68594: F, t68616: F, t68637: F, t942: F, t951: F, t959: F, t14473: F, t5804: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t68638, t68640, t68643, t68646, t68649, t68673) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2397::<F>(t13623, t5705, t17271, t4378, t21180, t2798, t896, t2815, t136, t68569, t908, t41684, t48946, t48947, t48956, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t68693 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398::<F>(t41904, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let (t68695, t68697, t68699) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2399::<F>(t68673, t68693, t894, t901, t59759, t59761, t60308, t60310, t60312, t68638, t68640, t68643, t68646, t68649);
        let (t68702, t68706, t68708) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2400::<F>(t68457, t68496, t68532, t68565, t68594, t68616, t68637, t68699, t942, t951, t959, t14473, t5804);
    (t68638, t68640, t68643, t68646, t68649, t68695, t68697, t68702, t68706, t68708)
}
