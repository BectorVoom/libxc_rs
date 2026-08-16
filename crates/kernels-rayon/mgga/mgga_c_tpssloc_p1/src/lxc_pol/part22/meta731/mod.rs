//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2397;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2399;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta731(t13623: f64, t5705: f64, t17271: f64, t4378: f64, t21180: f64, t2798: f64, t896: f64, t2815: f64, t136: f64, t68569: f64, t908: f64, t41684: f64, t48946: f64, t48947: f64, t48956: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t41904: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64, t894: f64, t901: f64, t60308: f64, t60310: f64, t60312: f64, t68457: f64, t68496: f64, t68532: f64, t68565: f64, t68594: f64, t68616: f64, t68637: f64, t942: f64, t951: f64, t959: f64, t14473: f64, t5804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68638, t68640, t68643, t68646, t68649, t68673) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2397(t13623, t5705, t17271, t4378, t21180, t2798, t896, t2815, t136, t68569, t908, t41684, t48946, t48947, t48956, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t68693 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398(t41904, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let (t68695, t68697, t68699) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2399(t68673, t68693, t894, t901, t59759, t59761, t60308, t60310, t60312, t68638, t68640, t68643, t68646, t68649);
        let (t68702, t68706, t68708) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2400(t68457, t68496, t68532, t68565, t68594, t68616, t68637, t68699, t942, t951, t959, t14473, t5804);
    (t68638, t68640, t68643, t68646, t68649, t68695, t68697, t68702, t68706, t68708)
}
