//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2147/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2147<F: Float>(t20054: F, t7132: F, t20050: F, t100092: F, t100097: F, t100117: F, t20066: F, t25577: F, t27493: F, t6323: F, t6327: F, t93611: F, t93618: F, t93622: F) -> F {
    let t106934 = t7132 * t20054;
    let t106938 = t7132 * t20050;
    let t106943 = F::cast_from(0.85748036236139473944e-3_f64) * t27493 * t20066 - F::cast_from(0.15244095330869239812e-2_f64) * t25577 * t6323 + F::cast_from(0.19055119163586549765e-3_f64) * t106934 - F::cast_from(0.2540682555144873302e-2_f64) * t25577 * t6327 + F::cast_from(0.31758531939310916275e-3_f64) * t106938 - t100092 + t100097 + t93611 + F::cast_from(0.50813651102897466041e-3_f64) * t93618 - F::cast_from(0.95275595817932748827e-4_f64) * t93622 + F::cast_from(0.38110238327173099531e-3_f64) * t100117;
    t106943
}
