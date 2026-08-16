//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 984/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk984(t525: f64, t879: f64, t7932: f64, t7942: f64, t2131: f64, t2147: f64, t309: f64, t8436: f64, t1221: f64, t2127: f64, t2146: f64, t2331: f64, t2347: f64, t29994: f64, t30030: f64, t30036: f64, t33488: f64, t33489: f64, t33496: f64, t33500: f64, t33504: f64, t33507: f64, t4109: f64, t7904: f64, t7931: f64, t7934: f64, t8004: f64, t9003: f64) -> (f64, f64) {
    let t33509 = t525 * t879;
    let t33511 = t7942 * t7932 * t33509;
    let t33516 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t8436 * t309;
    let t33517 = -0.26020884564615598386e1_f64 * t2146 * t8004 * t2331 * t1221 + 0.4336814094102599731e0_f64 * t9003 * t7904 - 0.17347256376410398924e1_f64 * t30030 - t30036 + t33488 - 0.17347256376410398924e1_f64 * t7931 * t33489 * t7934 + 0.4336814094102599731e0_f64 * t29994 * t2347 + t33496 - 0.39512695097613069591e1_f64 * t2127 * t4109 - t33500 + t33504 + 0.52041769129231196772e1_f64 * t33507 - 0.8673628188205199462e0_f64 * t33511 + t33516;
    (t33509, t33517)
}
