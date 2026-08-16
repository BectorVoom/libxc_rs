//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta805 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2793;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2794;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2795;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2796;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta805<F: Float>(t16558: F, t707: F, t751: F, t16586: F, t9929: F, t185: F, t55677: F, t16579: F, t172: F, t763: F, t67: F, t758: F, t59011: F, t59014: F, t59015: F, t59016: F, t59018: F, t59019: F, t59020: F, t59023: F, t59025: F, t59027: F, t59029: F, t59031: F, t59033: F, t59034: F, t59035: F, t12971: F, t13141: F, t13151: F, t13157: F, t13161: F, t13167: F, t1504: F, t1506: F, t16662: F, t16729: F, t16736: F, t16740: F, t16745: F, t16746: F, t225: F, t230: F, t2379: F, t2553: F, t2672: F, t4225: F, t4226: F, t5527: F, t5601: F, t58963: F, t58964: F, t58966: F, t58967: F, t58970: F, t58981: F, t59010: F, t6589: F, t776: F, t845: F, t232: F, t58947: F, t13184: F, t13193: F, t13210: F, t13251: F, t13265: F, t13302: F, t13350: F, t1510: F, t16891: F, t2643: F, t2684: F, t41116: F, t4172: F, t4180: F, t4234: F, t4250: F, t4255: F, t47039: F, t47044: F, t47047: F, t47049: F, t47079: F, t47081: F, t5619: F, t58890: F, t58900: F, t58904: F, t817: F, t819: F, t820: F, t9613: F, t16957: F, t41011: F, t213: F, t221: F, t41142: F, t41144: F, t41149: F, t41155: F, t41156: F, t41185: F, t41187: F, t41190: F, t41192: F, t41194: F, t41197: F, t4127: F, t46764: F, t46768: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t59038, t59040, t59043, t59046, t59048) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2793::<F>(t16558, t707, t751, t16586, t9929, t185, t55677, t16579, t172, t763, t67, t758);
        let (t59049, t59050) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2794::<F>(t59048, t59011, t59014, t59015, t59016, t59018, t59019, t59020, t59023, t59025, t59027, t59029, t59031, t59033, t59034, t59035, t59038, t59040, t59043, t59046);
        let t59072 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2795::<F>(t12971, t13141, t13151, t13157, t13161, t13167, t1504, t1506, t16662, t16729, t16736, t16740, t16745, t16746, t225, t230, t2379, t2553, t2672, t4225, t4226, t5527, t5601, t58963, t58964, t58966, t58967, t58970, t58981, t59010, t59050, t6589, t776, t845);
        let (t59074, t59088) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2796::<F>(t232, t58947, t59072, t13184, t13193, t13210, t13251, t13265, t13302, t13350, t1510, t16891, t2643, t2684, t41116, t4172, t4180, t4234, t4250, t4255, t47039, t47044, t47047, t47049, t47079, t47081, t5619, t58890, t58900, t58904, t817, t819, t820, t9613);
        let (t59100, t59134) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2797::<F>(t16957, t41011, t16662, t213, t221, t41142, t41144, t41149, t41155, t41156, t41185, t41187, t41190, t41192, t41194, t41197, t4127, t46764, t46768, t776);
    (t59038, t59040, t59043, t59046, t59049, t59074, t59088, t59100, t59134)
}
