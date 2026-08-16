//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2820/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2820(t14546: f64, t18525: f64, t39697: f64, t39701: f64, t39719: f64, t51424: f64, t51430: f64, t51435: f64, t51445: f64, t51452: f64, t62716: f64, t62723: f64, t76131: f64) -> f64 {
    let t76275 = 0.34697458558045176418e-2_f64 * t62716 - 0.34697458558045176418e-2_f64 * t62723 - 0.39029762157531132076e-2_f64 * t51424 + t51430 + t51435 + 0.91069445034239308177e-1_f64 * t51445 + t39697 + 0.33133632253434461091e-3_f64 * t51452 - 0.19637199382202157274e-3_f64 * t39701 - 0.11853808529283920877e2_f64 * t14546 * t76131 * t18525 + 0.19637199382202157274e-3_f64 * t39719;
    t76275
}
