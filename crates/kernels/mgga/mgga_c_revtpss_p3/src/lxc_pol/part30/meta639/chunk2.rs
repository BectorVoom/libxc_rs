//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2218/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2218<F: Float>(t5: F, t104194: F, t104222: F, t104249: F, t104274: F, t104303: F, t104330: F, t104359: F, t104403: F, t117: F, t101504: F, t101506: F, t101508: F, t101510: F, t101512: F, t101514: F, t101517: F, t101519: F, t101521: F, t101524: F, t101526: F, t101528: F, t104163: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t104407 = piecewise3::<F>(t8, F::new(0.0), t104194 + t104222 + t104249 + t104274 + t104303 + t104330 + t104359 + t104403);
    let t104408 = t104407 * t117;
    let t104409 = F::new(2.0) * t104163 + t104408 + t101504 + t101506 + t101508 + t101510 + t101512 + t101514 + t101517 + t101519 + t101521 + t101524 + t101526 + t101528;
    (t104408, t104409)
}
