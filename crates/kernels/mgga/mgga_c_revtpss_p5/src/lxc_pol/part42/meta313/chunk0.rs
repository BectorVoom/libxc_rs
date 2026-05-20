//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1085/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1085<F: Float>(t11239: F, t1243: F, t460: F, t3596: F, t13038: F, t1275: F, t225: F, t10270: F, t10272: F, t10279: F, t10281: F, t10288: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13126 = t11239 * t1243;
    let t13127 = t460 * t13126;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13180 = t1275 * t1275;
    let t13181 = F::new(1.0) / t13180;
    let t13182 = t225 * t13181;
    let t13261 = F::new(4.0) * t10270;
    let t13262 = F::new(12.0) * t10272;
    let t13263 = F::new(48.0) * t10279;
    let t13264 = F::new(80.0) * t10281;
    let t13265 = F::new(180.0) * t10288;
    (t13126, t13127, t13141, t13142, t13147, t13148, t13182, t13261, t13262, t13263, t13264, t13265)
}
