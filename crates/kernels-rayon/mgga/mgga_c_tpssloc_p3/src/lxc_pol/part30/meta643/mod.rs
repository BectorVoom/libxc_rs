//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2053;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta643(t23562: f64, t343: f64, t88405: f64, t1036: f64, t25622: f64, t14134: f64, t6765: f64, t1933: f64, t23479: f64, t88360: f64, t88365: f64, t25637: f64, t984: f64, t1014: f64, t82654: f64, t6722: f64, t1409: f64, t344: f64, t1009: f64, t6740: f64, t23473: f64, t3082: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88407, t88415, t88422, t88425, t88428, t88430) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2053(t23562, t343, t88405, t1036, t25622, t14134, t6765, t1933, t23479, t88360, t88365, t25637, t984);
        let (t88431, t88440, t88449, t88451, t88453, t88479) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2054(t1014, t82654, t23479, t25637, t6722, t1409, t344, t1009, t6740, t23473, t3082, t7586);
    (t88407, t88415, t88422, t88425, t88428, t88430, t88431, t88440, t88449, t88451, t88453, t88479)
}
