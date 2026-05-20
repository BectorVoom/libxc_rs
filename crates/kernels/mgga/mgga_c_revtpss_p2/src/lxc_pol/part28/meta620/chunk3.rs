//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2186/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2186<F: Float>(t2470: F, t27340: F, t25387: F, t1580: F, t25317: F, t25391: F, t25392: F, t25394: F, t27316: F, t27349: F, t7070: F, t886: F, t92864: F, t93186: F, t93276: F, t93278: F, t93283: F, t93286: F, t99334: F, t99342: F, t99344: F, t99346: F, t99351: F, t99360: F) -> (F, F) {
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    let t99368 = -F::cast_from(0.17347256376410398924e1_f64) * t25391 * t99334 * t25394 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t92864 * t27349 + t99342 - t93276 - t99344 + t99346 + t93278 + t99351 + F::cast_from(0.43368140941025997312e-1_f64) * t93283 - F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t27316 * t886 - F::cast_from(0.65854491829355115987e0_f64) * t93186 * t1580 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t25392 * t99360 + F::cast_from(0.38549458614245330943e-1_f64) * t93286 - F::cast_from(0.34270468708064099208e-1_f64) * t99366;
    (t99365, t99368)
}
