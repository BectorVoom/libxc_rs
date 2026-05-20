//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1126/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1126<F: Float>(t30: F, t33: F, t13302: F, t606: F, t2258: F, t4201: F, t580: F, t9342: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13303 = t13302 * t606;
    let t13306 = t4201 * t2258;
    let t13309 = F::new(2.0) * t580;
    let t13310 = F::new(6.0) * t9342;
    let t13312 = piecewise5::<F>(t31, F::new(0.0), t34, F::new(0.0), t13309 - t13310);
    (t13303, t13306, t13312)
}
