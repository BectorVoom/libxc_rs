//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1041/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1041<F: Float>(t10272: F, t10279: F, t10281: F, t10288: F, t10290: F, t4171: F, t602: F, t1466: F, t2246: F, t580: F, t9342: F, t116: F, t4245: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13262 = F::cast_from(12.0_f64) * t10272;
    let t13263 = F::cast_from(48.0_f64) * t10279;
    let t13264 = F::cast_from(80.0_f64) * t10281;
    let t13265 = F::cast_from(180.0_f64) * t10288;
    let t13266 = F::cast_from(252.0_f64) * t10290;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    let t13309 = F::cast_from(2.0_f64) * t580;
    let t13310 = F::cast_from(6.0_f64) * t9342;
    let t13426 = t4245 * t116;
    (t13262, t13263, t13264, t13265, t13266, t13269, t13272, t13309, t13310, t13426)
}
