//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 952/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk952<F: Float>(t12459: F, t12460: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F, t24313: F, t24315: F, t24318: F, t24320: F, t24348: F, t1169: F) -> (F,) {
    let t24361 = 0.309885e1 * t24242 + 0.516475e0 * t24250 - 0.20839e0 * t24289 + 0.62517e0 * t24292 + 0.104195e0 * t24295 - t12459 - t12460 - 0.104195e0 * t24298 - 0.103295e1 * t24238 + 0.309885e1 * t24246 + 0.6311625e0 * t24313 + 0.3529725e1 * t24315 + 0.264729375e1 * t24318 - 0.157790625e0 * t24320;
    let t24362 = t24348 + t24361;
    let t24363 = t24362 * t1169;
    (t24363,)
}
