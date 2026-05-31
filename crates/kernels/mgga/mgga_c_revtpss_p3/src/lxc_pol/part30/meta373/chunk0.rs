//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1400/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1400<F: Float>(t13126: F, t460: F, t3727: F, t473: F, t11239: F, t3596: F, t13038: F, t1269: F, t3555: F, t1275: F, t225: F, t10270: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13127 = t460 * t13126;
    let t13133 = t473 * t3727;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13177 = t3555 * t1269;
    let t13180 = t1275 * t1275;
    let t13181 = F::cast_from(1.0_f64) / t13180;
    let t13182 = t225 * t13181;
    let t13261 = F::cast_from(4.0_f64) * t10270;
    (t13127, t13133, t13141, t13142, t13147, t13148, t13177, t13180, t13181, t13182, t13261)
}
