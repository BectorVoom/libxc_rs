//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 893/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk893<F: Float>(t163: F, t169: F, t299: F, t3569: F, t1: F, t3: F, t3379: F, t672: F, t10290: F, t10291: F, t10294: F, t10299: F, t10302: F, t10303: F, t10305: F, t4910: F, t7045: F, t7047: F, t8404: F, t8405: F, t8408: F, t8413: F, t8414: F) -> (F, F) {
    let t11187 = t169 * t299 * t3569 * t163;
    let t11190 = t3379 * t1 * t3;
    let t11191 = t11190 * t672;
    let t11196 = 0.10821041362364843377e0 * t11191 + t10290 + t4910 + t8404 + 8.0 / 3.0 * t8405 - t10291 - t7045 + t7047 + 0.14428055149819791169e0 * t8408 + t8413 + 0.43284165449459373508e0 * t8414 + t10294 + t10299 - t10302 - t10303 - t10305;
    (t11187, t11196)
}
