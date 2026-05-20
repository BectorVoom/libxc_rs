//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1930/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1930<F: Float>(t2435: F, t28448: F, t28314: F, t93364: F, t103431: F, t25375: F, t212: F, t28340: F, t689: F, t780: F, t103182: F, t93281: F) -> (F, F, F, F, F) {
    let t103490 = t2435 * t28448;
    let t103494 = F::cast_from(0.28912093960683998208e-1_f64) * t93364 * t28314;
    let t103521 = t25375 * t103431;
    let t103529 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t212 * t28340 * t780;
    let t103543 = t93281 * t103182;
    (t103490, t103494, t103521, t103529, t103543)
}
