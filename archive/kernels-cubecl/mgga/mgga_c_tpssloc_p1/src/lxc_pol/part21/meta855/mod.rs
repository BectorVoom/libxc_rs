//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta855 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3090;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3091;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3092;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3093;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3094;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta855<F: Float>(t51043: F, t51051: F, t51053: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t18502: F, t699: F, t18499: F, t136: F, t3297: F, t63394: F, t63386: F, t63390: F, t18509: F, t18507: F, t1113: F, t63410: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t1099: F, t1118: F, t63847: F, t63881: F, t63916: F, t64011: F, t64027: F, t64049: F, t3356: F, t6031: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63841: F, t63843: F, t63845: F, t63323: F, t63327: F, t63330: F, t63848: F, t63853: F, t63856: F, t63858: F, t63860: F, t63862: F, t63865: F, t63867: F, t63870: F, t63873: F, t63876: F, t63879: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t64066 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3090::<F>(t51043, t51051, t51053, t63355, t63359, t63361, t63365, t63370, t63374, t63380, t63382, t63384, t63388, t63392, t63396);
        let (t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3091::<F>(t18502, t699, t18499, t136, t3297, t63394, t63386, t63390, t18509, t18507, t1113, t63410);
        let t64094 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3092::<F>(t63398, t63400, t63404, t63408, t63412, t63417, t63422, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let (t64100, t64103) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3093::<F>(t1099, t1118, t63847, t63881, t63916, t64011, t64027, t64049, t64066, t64094, t3356, t6031);
        let t64132 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3094::<F>(t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63841, t63843, t63845);
        let t64148 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3095::<F>(t63323, t63327, t63330, t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867, t63870, t63873, t63876, t63879);
    (t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092, t64100, t64103, t64132, t64148)
}
