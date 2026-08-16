//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1949;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta662(t28299: f64, t81979: f64, t28273: f64, t6547: f64, t28264: f64, t17022: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t28272: f64, t6562: f64, t794: f64, t25224: f64, t25341: f64, t6552: f64, t23164: f64, t7479: f64, t86893: f64, t16596: f64, t86721: f64, t1484: f64, t584: f64, t86753: f64, t16949: f64, t25014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98993, t98995, t99003, t99019, t99022) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1949(t28299, t81979, t28273, t6547, t28264, t17022, t1880, t214, t225, t258, t28272, t6562, t794);
        let (t99033, t99036, t99049, t99053, t99056) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1950(t25224, t25341, t6552, t23164, t7479, t86893, t16596, t86721, t1484, t584, t86753, t16949, t25014);
    (t98993, t98995, t99003, t99019, t99022, t99033, t99036, t99049, t99053, t99056)
}
