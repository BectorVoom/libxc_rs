//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta853 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3082;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3083;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3084;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3085;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta853<F: Float>(t136: F, t43761: F, t63420: F, t3297: F, t63311: F, t63315: F, t63368: F, t11219: F, t63372: F, t63378: F, t1113: F, t63402: F, t63406: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63323: F, t43748: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t63327: F, t63330: F, t63332: F, t63334: F, t63336: F, t43780: F, t43782: F, t43816: F, t43820: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63918, t63921, t63924, t63927, t63930, t63933, t63936) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3082::<F>(t136, t43761, t63420, t3297, t63311, t63315, t63368, t11219, t63372, t63378, t1113, t63402);
        let (t63939, t63953) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3083::<F>(t1113, t136, t63406, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63967 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3084::<F>(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let t63980 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3085::<F>(t43780, t43782, t43816, t43820, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let t63994 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3086::<F>(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
    (t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939, t63953, t63967, t63980, t63994)
}
