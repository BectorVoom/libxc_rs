//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 982/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk982(t7987: f64, t8423: f64, t157: f64, t2146: f64, t2152: f64, t29992: f64, t30006: f64, t30011: f64, t30015: f64, t33444: f64, t33451: f64, t33459: f64, t33465: f64, t33468: f64, t463: f64, t5079: f64, t524: f64, t609: f64, t7877: f64, t7954: f64, t8004: f64, t8010: f64, t8436: f64, t9003: f64) -> f64 {
    let t33475 = 0.17347256376410398924e1_f64 * t7987 * t8423;
    let t33478 = 0.8673628188205199462e0_f64 * t33444 - 0.17347256376410398924e1_f64 * t29992 + 0.17347256376410398924e1_f64 * t9003 * t8010 - 0.17347256376410398924e1_f64 * t33451 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t609 * t5079 * t157 + t33459 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t7877 * t524 * t157 + 0.8673628188205199462e0_f64 * t33465 + t33468 + 0.34694512752820797848e1_f64 * t30006 + t30011 + t30015 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t8436 * t463 - t33475 + 0.4336814094102599731e0_f64 * t9003 * t7954;
    t33478
}
