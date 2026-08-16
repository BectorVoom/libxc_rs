//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 989/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk989(t163: f64, t169: f64, t299: f64, t3569: f64, t1: f64, t3: f64, t3379: f64, t672: f64, t10290: f64, t10291: f64, t10294: f64, t10299: f64, t10302: f64, t10303: f64, t10305: f64, t4910: f64, t7045: f64, t7047: f64, t8404: f64, t8405: f64, t8408: f64, t8413: f64, t8414: f64) -> (f64, f64) {
    let t11187 = t169 * t299 * t3569 * t163;
    let t11190 = t3379 * t1 * t3;
    let t11191 = t11190 * t672;
    let t11196 = 0.10821041362364843377e0_f64 * t11191 + t10290 + t4910 + t8404 + 8.0_f64 / 3.0_f64 * t8405 - t10291 - t7045 + t7047 + 0.14428055149819791169e0_f64 * t8408 + t8413 + 0.43284165449459373508e0_f64 * t8414 + t10294 + t10299 - t10302 - t10303 - t10305;
    (t11187, t11196)
}
