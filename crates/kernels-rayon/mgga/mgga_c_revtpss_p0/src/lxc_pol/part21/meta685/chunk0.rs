//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2501/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2501(t12861: f64, t12916: f64, t3718: f64, t11262: f64, t3600: f64, t3605: f64, t1261: f64, t12925: f64, t3172: f64, t12921: f64, t3711: f64, t3617: f64, t675: f64) -> (f64, f64, f64, f64, f64) {
    let t44672 = t3718 * t12916 * t12861;
    let t44675 = t3600 * t11262 * t3605;
    let t44678 = t1261 * t3172 * t12925;
    let t44681 = t3711 * t3172 * t12921;
    let t44693 = t675 * t3617;
    (t44672, t44675, t44678, t44681, t44693)
}
