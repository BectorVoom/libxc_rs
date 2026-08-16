//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 836/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk836<F: Float>(t150: F, t187: F, t9367: F, t119: F, t2146: F, t639: F, t7931: F, t8098: F, t8101: F, t8106: F, t8113: F, t8115: F, t9058: F, t9150: F, t9155: F, t9160: F, t9162: F, t9165: F, t9169: F, t9172: F) -> (F, F) {
    let t9369 = t9367 * t150 * t187;
    let t9375 = -F::cast_from(0.26020884564615598386e1_f64) * t2146 * t9150 - F::cast_from(0.17347256376410398924e1_f64) * t9155 - t8098 - F::cast_from(0.8673628188205199462e0_f64) * t8101 - t8106 + F::cast_from(0.17347256376410398924e1_f64) * t9160 - F::cast_from(0.8673628188205199462e0_f64) * t7931 * t9162 - F::cast_from(0.8673628188205199462e0_f64) * t7931 * t9165 + F::cast_from(0.8673628188205199462e0_f64) * t9169 - F::cast_from(0.8673628188205199462e0_f64) * t9172 - t8113 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t9369 - F::cast_from(0.65854491829355115987e0_f64) * t8115 - F::cast_from(0.4336814094102599731e0_f64) * t9058 * t639;
    (t9369, t9375)
}
