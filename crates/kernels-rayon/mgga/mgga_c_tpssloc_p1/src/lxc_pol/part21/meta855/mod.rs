//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta855 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3090;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3091;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3092;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3093;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3094;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta855(t51043: f64, t51051: f64, t51053: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t18502: f64, t699: f64, t18499: f64, t136: f64, t3297: f64, t63394: f64, t63386: f64, t63390: f64, t18509: f64, t18507: f64, t1113: f64, t63410: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64, t1099: f64, t1118: f64, t63847: f64, t63881: f64, t63916: f64, t64011: f64, t64027: f64, t64049: f64, t3356: f64, t6031: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63841: f64, t63843: f64, t63845: f64, t63323: f64, t63327: f64, t63330: f64, t63848: f64, t63853: f64, t63856: f64, t63858: f64, t63860: f64, t63862: f64, t63865: f64, t63867: f64, t63870: f64, t63873: f64, t63876: f64, t63879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t64066 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3090(t51043, t51051, t51053, t63355, t63359, t63361, t63365, t63370, t63374, t63380, t63382, t63384, t63388, t63392, t63396);
        let (t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3091(t18502, t699, t18499, t136, t3297, t63394, t63386, t63390, t18509, t18507, t1113, t63410);
        let t64094 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3092(t63398, t63400, t63404, t63408, t63412, t63417, t63422, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let (t64100, t64103) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3093(t1099, t1118, t63847, t63881, t63916, t64011, t64027, t64049, t64066, t64094, t3356, t6031);
        let t64132 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3094(t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63841, t63843, t63845);
        let t64148 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3095(t63323, t63327, t63330, t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867, t63870, t63873, t63876, t63879);
    (t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092, t64100, t64103, t64132, t64148)
}
