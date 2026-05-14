//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1172/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1172<F: Float>(t114401: F, t1312: F, t105866: F, t114360: F, t114363: F, t114372: F, t114375: F, t114377: F, t114380: F, t114382: F, t114384: F, t114387: F, t114389: F, t114391: F, t1518: F, t22633: F, t28030: F, t33602: F, t5920: F, t6985: F) -> (F,) {
    let t114403 = 2.0 * t1312 * t114401;
    let t114404 = 6.0 * t105866 * t1518 + 2.0 * t22633 * t6985 + 6.0 * t28030 * t5920 + 6.0 * t33602 * t5920 + t114360 + 6.0 * t114363 + t114372 + t114375 + t114377 + t114380 + t114382 + t114384 + t114387 + t114389 + t114391 + t114403;
    (t114404,)
}
