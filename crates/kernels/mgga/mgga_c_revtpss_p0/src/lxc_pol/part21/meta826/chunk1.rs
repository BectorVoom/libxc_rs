//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3079/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3079<F: Float>(t1149: F, t16943: F, t3384: F, t16942: F, t3433: F, t3435: F, t56262: F, t56264: F, t56268: F, t56271: F, t56275: F, t56277: F, t56279: F, t56281: F, t56283: F) -> (F, F, F) {
    let t56286 = F::cast_from(6.0_f64) * t3384 * t16943 * t1149;
    let t56290 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t16942 * t3435 * t1149;
    let t56291 = t56262 - t56264 + t56268 + t56271 + t56275 - t56277 + t56279 - t56281 + t56283 - t56286 + t56290;
    (t56286, t56290, t56291)
}
