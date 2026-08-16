//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1974/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1974(t1358: f64, t212: f64, t30247: f64, t689: f64, t102241: f64, t102246: f64, t102249: f64, t102253: f64, t108225: f64, t108371: f64, t108502: f64, t1882: f64, t2103: f64, t25921: f64, t25930: f64, t28855: f64, t28888: f64, t28911: f64, t30262: f64, t543: f64, t7295: f64, t7301: f64, t8095: f64, t96257: f64, t96260: f64, t96265: f64, t98050: f64) -> f64 {
    let t109488 = t689 * t212 * t30247 * t1358;
    let t109493 = t102241 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t28888 * t1882 * t543 - t96257 - 0.22849835011101738147e-2_f64 * t96260 - t102246 + 0.8673628188205199462e0_f64 * t108225 * t28855 + 0.17347256376410398924e1_f64 * t98050 * t8095 - 0.34270468708064099208e-1_f64 * t96265 + 0.4336814094102599731e0_f64 * t25921 * t30262 - 0.14634331517634470219e-1_f64 * t102249 - 0.4336814094102599731e0_f64 * t108371 * t2103 - 0.54878743191129263322e-2_f64 * t109488 + 0.17347256376410398924e1_f64 * t25930 * t28911 * t108502 + t102253;
    t109493
}
