//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1841;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta381(t13931: f64, t340: f64, t343: f64, t974: f64, t10263: f64, t10287: f64, t10290: f64, t10331: f64, t10333: f64, t10339: f64, t10342: f64, t10353: f64, t13896: f64, t13907: f64, t13909: f64, t13915: f64, t1600: f64, t2960: f64, t4543: f64, t973: f64, t13804: f64, t13845: f64, t13894: f64, t225: f64, t68: f64, t369: f64, t1036: f64, t4622: f64, t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64, t1041: f64, t10370: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t10390: f64, t13750: f64, t13751: f64, t13758: f64, t13762: f64, t13767: f64, t3070: f64, t378: f64, t4579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13933, t13937) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1840(t13931, t340, t343, t974, t10263, t10287, t10290, t10331, t10333, t10339, t10342, t10353, t13896, t13907, t13909, t13915, t1600, t2960, t4543, t973);
        let (t13939, t13940, t13941, t13942, t13946, t13948, t13950) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1841(t13804, t13845, t13894, t13937, t225, t68, t369, t1036, t4622, t3117, t4571, t248, t3051, t4347);
        let (t13952, t13953) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1842(t1041, t13950, t10370, t10372, t10377, t10381, t10385, t10390, t13750, t13751, t13758, t13762, t13767, t13942, t13946, t13948, t3070, t378, t4579);
    (t13933, t13939, t13940, t13941, t13942, t13946, t13948, t13950, t13952, t13953)
}
