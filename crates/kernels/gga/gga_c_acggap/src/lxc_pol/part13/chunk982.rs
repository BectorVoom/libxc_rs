//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 982/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk982<F: Float>(t7987: F, t8423: F, t157: F, t2146: F, t2152: F, t29992: F, t30006: F, t30011: F, t30015: F, t33444: F, t33451: F, t33459: F, t33465: F, t33468: F, t463: F, t5079: F, t524: F, t609: F, t7877: F, t7954: F, t8004: F, t8010: F, t8436: F, t9003: F) -> F {
    let t33475 = F::cast_from(0.17347256376410398924e1_f64) * t7987 * t8423;
    let t33478 = F::cast_from(0.8673628188205199462e0_f64) * t33444 - F::cast_from(0.17347256376410398924e1_f64) * t29992 + F::cast_from(0.17347256376410398924e1_f64) * t9003 * t8010 - F::cast_from(0.17347256376410398924e1_f64) * t33451 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2152 * t609 * t5079 * t157 + t33459 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2152 * t7877 * t524 * t157 + F::cast_from(0.8673628188205199462e0_f64) * t33465 + t33468 + F::cast_from(0.34694512752820797848e1_f64) * t30006 + t30011 + t30015 - F::cast_from(0.52041769129231196772e1_f64) * t2146 * t8004 * t8436 * t463 - t33475 + F::cast_from(0.4336814094102599731e0_f64) * t9003 * t7954;
    t33478
}
