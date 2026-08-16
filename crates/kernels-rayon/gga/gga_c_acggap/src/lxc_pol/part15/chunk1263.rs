//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1263/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1263(t31797: f64, t32942: f64, t36231: f64, t36236: f64, t36238: f64, t37894: f64, t37898: f64, t37899: f64, t37904: f64, t37905: f64, t37907: f64, t37908: f64, t37909: f64, t37910: f64, t37924: f64, t40450: f64, t40455: f64, t40458: f64) -> f64 {
    let t42118 = t37894 - t37898 - t37899 - t37904 + t37905 - t37907 - t37908 - t37909 - t37910 + 0.18868855373762491241e-1_f64 * t40450 - 0.31448092289604152069e-3_f64 * t31797 - t32942 - 0.18140473443734395377e0_f64 * t36231 + 0.90702367218671976884e-1_f64 * t36236 - 0.38110238327173099531e-2_f64 * t36238 - t37924 - 0.42874018118069736972e-2_f64 * t40455 - 0.25724410870841842183e-2_f64 * t40458;
    t42118
}
