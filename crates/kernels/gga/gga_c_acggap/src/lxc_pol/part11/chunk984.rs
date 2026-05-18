//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 984/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk984<F: Float>(t525: F, t879: F, t7932: F, t7942: F, t2131: F, t2147: F, t309: F, t8436: F, t1221: F, t2127: F, t2146: F, t2331: F, t2347: F, t29994: F, t30030: F, t30036: F, t33488: F, t33489: F, t33496: F, t33500: F, t33504: F, t33507: F, t4109: F, t7904: F, t7931: F, t7934: F, t8004: F, t9003: F) -> (F, F) {
    let t33509 = t525 * t879;
    let t33511 = t7942 * t7932 * t33509;
    let t33516 = F::new(0.34694512752820797848e1) * t2131 * t2147 * t8436 * t309;
    let t33517 = -F::new(0.26020884564615598386e1) * t2146 * t8004 * t2331 * t1221 + F::new(0.4336814094102599731e0) * t9003 * t7904 - F::new(0.17347256376410398924e1) * t30030 - t30036 + t33488 - F::new(0.17347256376410398924e1) * t7931 * t33489 * t7934 + F::new(0.4336814094102599731e0) * t29994 * t2347 + t33496 - F::new(0.39512695097613069591e1) * t2127 * t4109 - t33500 + t33504 + F::new(0.52041769129231196772e1) * t33507 - F::new(0.8673628188205199462e0) * t33511 + t33516;
    (t33509, t33517)
}
