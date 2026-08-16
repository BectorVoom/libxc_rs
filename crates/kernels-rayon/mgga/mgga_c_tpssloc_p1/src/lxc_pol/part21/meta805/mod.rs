//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta805 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2793;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2794;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2795;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2796;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta805(t16558: f64, t707: f64, t751: f64, t16586: f64, t9929: f64, t185: f64, t55677: f64, t16579: f64, t172: f64, t763: f64, t67: f64, t758: f64, t59011: f64, t59014: f64, t59015: f64, t59016: f64, t59018: f64, t59019: f64, t59020: f64, t59023: f64, t59025: f64, t59027: f64, t59029: f64, t59031: f64, t59033: f64, t59034: f64, t59035: f64, t12971: f64, t13141: f64, t13151: f64, t13157: f64, t13161: f64, t13167: f64, t1504: f64, t1506: f64, t16662: f64, t16729: f64, t16736: f64, t16740: f64, t16745: f64, t16746: f64, t225: f64, t230: f64, t2379: f64, t2553: f64, t2672: f64, t4225: f64, t4226: f64, t5527: f64, t5601: f64, t58963: f64, t58964: f64, t58966: f64, t58967: f64, t58970: f64, t58981: f64, t59010: f64, t6589: f64, t776: f64, t845: f64, t232: f64, t58947: f64, t13184: f64, t13193: f64, t13210: f64, t13251: f64, t13265: f64, t13302: f64, t13350: f64, t1510: f64, t16891: f64, t2643: f64, t2684: f64, t41116: f64, t4172: f64, t4180: f64, t4234: f64, t4250: f64, t4255: f64, t47039: f64, t47044: f64, t47047: f64, t47049: f64, t47079: f64, t47081: f64, t5619: f64, t58890: f64, t58900: f64, t58904: f64, t817: f64, t819: f64, t820: f64, t9613: f64, t16957: f64, t41011: f64, t213: f64, t221: f64, t41142: f64, t41144: f64, t41149: f64, t41155: f64, t41156: f64, t41185: f64, t41187: f64, t41190: f64, t41192: f64, t41194: f64, t41197: f64, t4127: f64, t46764: f64, t46768: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59038, t59040, t59043, t59046, t59048) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2793(t16558, t707, t751, t16586, t9929, t185, t55677, t16579, t172, t763, t67, t758);
        let (t59049, t59050) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2794(t59048, t59011, t59014, t59015, t59016, t59018, t59019, t59020, t59023, t59025, t59027, t59029, t59031, t59033, t59034, t59035, t59038, t59040, t59043, t59046);
        let t59072 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2795(t12971, t13141, t13151, t13157, t13161, t13167, t1504, t1506, t16662, t16729, t16736, t16740, t16745, t16746, t225, t230, t2379, t2553, t2672, t4225, t4226, t5527, t5601, t58963, t58964, t58966, t58967, t58970, t58981, t59010, t59050, t6589, t776, t845);
        let (t59074, t59088) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2796(t232, t58947, t59072, t13184, t13193, t13210, t13251, t13265, t13302, t13350, t1510, t16891, t2643, t2684, t41116, t4172, t4180, t4234, t4250, t4255, t47039, t47044, t47047, t47049, t47079, t47081, t5619, t58890, t58900, t58904, t817, t819, t820, t9613);
        let (t59100, t59134) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2797(t16957, t41011, t16662, t213, t221, t41142, t41144, t41149, t41155, t41156, t41185, t41187, t41190, t41192, t41194, t41197, t4127, t46764, t46768, t776);
    (t59038, t59040, t59043, t59046, t59049, t59074, t59088, t59100, t59134)
}
