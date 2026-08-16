//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta853 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3082;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3083;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3084;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3085;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta853(t136: f64, t43761: f64, t63420: f64, t3297: f64, t63311: f64, t63315: f64, t63368: f64, t11219: f64, t63372: f64, t63378: f64, t1113: f64, t63402: f64, t63406: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63323: f64, t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64, t43780: f64, t43782: f64, t43816: f64, t43820: f64, t50952: f64, t50954: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63918, t63921, t63924, t63927, t63930, t63933, t63936) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3082(t136, t43761, t63420, t3297, t63311, t63315, t63368, t11219, t63372, t63378, t1113, t63402);
        let (t63939, t63953) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3083(t1113, t136, t63406, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63967 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3084(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let t63980 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3085(t43780, t43782, t43816, t43820, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let t63994 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3086(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
    (t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939, t63953, t63967, t63980, t63994)
}
