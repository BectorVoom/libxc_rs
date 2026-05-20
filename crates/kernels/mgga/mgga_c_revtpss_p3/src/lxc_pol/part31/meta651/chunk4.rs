//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2156/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2156<F: Float>(t19907: F, t7111: F, t100327: F, t100329: F, t100332: F, t100334: F, t100336: F, t100342: F, t100343: F, t18909: F, t18926: F, t18930: F, t27526: F, t27527: F, t27531: F) -> F {
    let t107154 = t7111 * t19907;
    let t107159 = t27526 * t27527 * t18926 / F::new(48.0) - t27526 * t27527 * t18930 / F::new(72.0) - t27526 * t27531 * t18909 / F::new(36.0) + t107154 / F::new(864.0) + F::cast_from(0.30488190661738479625e-2_f64) * t100327 + F::cast_from(0.19055119163586549765e-3_f64) * t100329 - t100332 - t100334 - t100336 - t100342 - F::cast_from(0.1270341277572436651e-3_f64) * t100343;
    t107159
}
