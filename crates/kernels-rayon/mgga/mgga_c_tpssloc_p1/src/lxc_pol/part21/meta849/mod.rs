//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta849 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3072;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3073;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3074;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3075;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3076;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta849(t1671: f64, t51397: f64, t18786: f64, t3371: f64, t63717: f64, t63720: f64, t63722: f64, t63725: f64, t63729: f64, t63731: f64, t63733: f64, t63735: f64, t63737: f64, t63739: f64, t63741: f64, t63743: f64, t63745: f64, t63747: f64, t63752: f64, t63754: f64, t63757: f64, t14845: f64, t4782: f64, t14914: f64, t4740: f64, t44159: f64, t5989: f64, t11180: f64, t6021: f64, t18835: f64, t3259: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63323: f64, t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64, t43780: f64, t43782: f64, t43816: f64, t44348: f64, t50952: f64, t50954: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64, t423: f64, t18496: f64, t699: f64, t18517: f64, t18514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63759, t63760) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3072(t1671, t51397, t18786, t3371, t63717, t63720, t63722, t63725, t63729, t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757);
        let (t63763, t63765, t63767, t63769, t63771, t63784) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3073(t14845, t4782, t14914, t4740, t44159, t5989, t11180, t6021, t18835, t3259, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63798 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3074(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let t63811 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3075(t43780, t43782, t43816, t44348, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let t63825 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3076(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
        let (t63829, t63841, t63843, t63845, t63847) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3077(t423, t63784, t63798, t63811, t63825, t18496, t699, t18517, t18514, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317);
    (t63759, t63760, t63763, t63765, t63767, t63769, t63771, t63829, t63841, t63843, t63845, t63847)
}
