//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1811;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1812;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta409(t13804: f64, t13845: f64, t13894: f64, t13937: f64, t225: f64, t68: f64, t369: f64, t1036: f64, t4622: f64, t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64, t1041: f64, t10370: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t10390: f64, t13750: f64, t13751: f64, t13758: f64, t13762: f64, t13767: f64, t3070: f64, t378: f64, t4579: f64, t3114: f64, t4630: f64, t3101: f64, t4650: f64, t1020: f64, t10508: f64, t1616: f64, t122: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13939, t13940, t13941, t13942, t13946, t13948, t13950) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1811(t13804, t13845, t13894, t13937, t225, t68, t369, t1036, t4622, t3117, t4571, t248, t3051, t4347);
        let t13953 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1812(t1041, t13950, t10370, t10372, t10377, t10381, t10385, t10390, t13750, t13751, t13758, t13762, t13767, t13942, t13946, t13948, t3070, t378, t4579);
        let (t13959, t13961, t13963, t13965, t13966, t13969) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1813(t3114, t4630, t248, t3101, t4650, t1020, t10508, t1616, t122, t247);
    (t13939, t13940, t13941, t13942, t13950, t13953, t13959, t13961, t13963, t13965, t13966, t13969)
}
