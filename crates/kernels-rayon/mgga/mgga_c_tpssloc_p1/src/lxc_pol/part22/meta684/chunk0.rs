//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2250/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2250(t1036: f64, t18010: f64, t14025: f64, t14508: f64, t13970: f64, t14511: f64, t1057: f64, t61729: f64, t3199: f64, t61734: f64, t3185: f64, t18053: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62893 = t18010 * t1036;
    let t62901 = t14508 * t14025;
    let t62903 = t14511 * t13970;
    let t62994 = t61729 * t1057;
    let t63004 = t61734 * t3199;
    let t63183 = t61734 * t3185;
    let t63215 = t18053 * t225;
    (t62893, t62901, t62903, t62994, t63004, t63183, t63215)
}
