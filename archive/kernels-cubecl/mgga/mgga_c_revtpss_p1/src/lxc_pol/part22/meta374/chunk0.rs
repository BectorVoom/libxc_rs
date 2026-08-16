//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1923/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1923<F: Float>(t13147: F, t460: F, t1269: F, t3555: F, t1275: F, t225: F, t10270: F, t10272: F, t10279: F, t10281: F, t10288: F, t10290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13148 = t460 * t13147;
    let t13177 = t3555 * t1269;
    let t13180 = t1275 * t1275;
    let t13181 = F::cast_from(1.0_f64) / t13180;
    let t13182 = t225 * t13181;
    let t13261 = F::cast_from(4.0_f64) * t10270;
    let t13262 = F::cast_from(12.0_f64) * t10272;
    let t13263 = F::cast_from(48.0_f64) * t10279;
    let t13264 = F::cast_from(80.0_f64) * t10281;
    let t13265 = F::cast_from(180.0_f64) * t10288;
    let t13266 = F::cast_from(252.0_f64) * t10290;
    (t13148, t13177, t13180, t13181, t13182, t13261, t13262, t13263, t13264, t13265, t13266)
}
