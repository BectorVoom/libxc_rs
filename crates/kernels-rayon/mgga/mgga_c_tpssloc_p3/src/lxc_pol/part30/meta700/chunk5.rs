//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2258/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2258(t23110: f64, t23185: f64, t28321: f64, t16805: f64, t1909: f64, t226: f64, t235: f64, t25256: f64, t28407: f64, t4166: f64, t4291: f64, t808: f64, t812: f64, t82032: f64, t82039: f64, t82047: f64, t829: f64, t87710: f64, t87714: f64, t87730: f64, t87734: f64, t92817: f64, t98524: f64, t98592: f64, t98601: f64, t98608: f64, t98876: f64, t98881: f64) -> f64 {
    let t98884 = t23185 * t23110 * t28321;
    let t98886 = -2.0_f64 * t4291 * t98524 * t829 + t808 * t28407 - t812 * t98592 * t829 - 2.0_f64 * t4166 * t25256 - 0.26044789391763585244e-1_f64 * t82032 - 0.16449340668482264365e-1_f64 * t98601 - 0.52089578783527170488e-1_f64 * t82039 + t87710 - 0.49348022005446793095e-1_f64 * t87714 + t16805 * t1909 - t82047 + 0.3289868133696452873e-1_f64 * t98608 - t92817 + t226 * t235 * t98876 + t87730 + 0.49348022005446793095e-1_f64 * t98881 + 0.41123351671205660912e-2_f64 * t98884 - t87734;
    t98886
}
