//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 599/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk599<F: Float>(t3316: F, t342: F, t3303: F, t357: F, t389: F, t1941: F, t268: F, t404: F) -> (F, F, F, F, F) {
    let t3317 = t342 * t3316;
    let t3318 = t3303 * t357;
    let t3335 = t389 * t389;
    let t3336 = F::cast_from(1.0_f64) / t3335;
    let t3356 = t268 * t1941 * t404;
    (t3317, t3318, t3335, t3336, t3356)
}
