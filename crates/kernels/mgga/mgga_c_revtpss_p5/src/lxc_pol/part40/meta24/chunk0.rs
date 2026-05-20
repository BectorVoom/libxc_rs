//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 155/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk155<F: Float>(t221: F, t462: F, t65: F, t225: F, t460: F, t355: F, t424: F, t452: F, t454: F) -> (F, F, F) {
    let t464 = t221 * t65 * t462;
    let t467 = t460 * t225;
    let t471 = F::exp(-(-t424 + t452 + t454) * t225 * t355);
    (t464, t467, t471)
}
