//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 813/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk813(t150: f64, t187: f64, t9367: f64, t119: f64, t2146: f64, t639: f64, t7931: f64, t8098: f64, t8101: f64, t8106: f64, t8113: f64, t8115: f64, t9058: f64, t9150: f64, t9155: f64, t9160: f64, t9162: f64, t9165: f64, t9169: f64, t9172: f64) -> (f64, f64) {
    let t9369 = t9367 * t150 * t187;
    let t9375 = -0.26020884564615598386e1_f64 * t2146 * t9150 - 0.17347256376410398924e1_f64 * t9155 - t8098 - 0.8673628188205199462e0_f64 * t8101 - t8106 + 0.17347256376410398924e1_f64 * t9160 - 0.8673628188205199462e0_f64 * t7931 * t9162 - 0.8673628188205199462e0_f64 * t7931 * t9165 + 0.8673628188205199462e0_f64 * t9169 - 0.8673628188205199462e0_f64 * t9172 - t8113 + 0.65854491829355115987e0_f64 * t119 * t9369 - 0.65854491829355115987e0_f64 * t8115 - 0.4336814094102599731e0_f64 * t9058 * t639;
    (t9369, t9375)
}
