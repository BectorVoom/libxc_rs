//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2831/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2831<F: Float>(t10811: F, t23297: F, t14923: F, t23336: F, t14894: F, t18525: F, t40455: F, t40489: F, t4364: F, t50436: F, t50443: F, t50454: F, t50457: F, t50505: F, t50524: F, t61689: F, t61692: F, t61697: F, t61699: F, t61718: F, t61727: F, t61754: F, t76284: F) -> F {
    let t76500 = t10811 * t23297;
    let t76502 = t14923 * t23336;
    let t76517 = F::cast_from(0.85748036236139473945e-3_f64) * t61689 - F::cast_from(0.17149607247227894789e-3_f64) * t61692 - F::cast_from(0.76230004213927992338e-3_f64) * t61697 + F::cast_from(0.13605355082800796532e0_f64) * t61699 + F::cast_from(0.3001181268264881588e-2_f64) * t76500 + F::cast_from(0.24009450146119052705e-1_f64) * t76502 - F::cast_from(0.77173232612525526552e-2_f64) * t14894 * t4364 * t76284 * t18525 - F::cast_from(0.60246173160355784831e-6_f64) * t50436 + F::cast_from(0.45732285992607719436e-3_f64) * t50443 - t50454 + F::cast_from(0.27107389498472794075e-3_f64) * t50457 + F::cast_from(0.76230004213927992336e-4_f64) * t61718 + F::cast_from(0.15246000842785598468e-2_f64) * t61727 - F::cast_from(0.80328230880474379776e-6_f64) * t40455 + F::cast_from(0.72250660161932334527e-3_f64) * t40489 - t50505 + F::cast_from(0.12004725073059526352e0_f64) * t61754 + F::cast_from(0.34013387707001991332e0_f64) * t50524;
    t76517
}
