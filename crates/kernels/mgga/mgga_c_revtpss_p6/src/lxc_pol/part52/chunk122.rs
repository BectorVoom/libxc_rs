//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 122/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk122<F: Float>(t344: F, t56: F, t404: F, t221: F, t65: F, t225: F, t460: F, t355: F, t424: F, t452: F, t454: F) -> (F, F, F, F, F) {
    let t461 = t56 * t344;
    let t462 = F::cast_from(1.0_f64) / t404;
    let t464 = t221 * t65 * t462;
    let t467 = t460 * t225;
    let t471 = F::exp(-(-t424 + t452 + t454) * t225 * t355);
    (t461, t462, t464, t467, t471)
}
