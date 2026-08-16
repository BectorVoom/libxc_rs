//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 712/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk712<F: Float>(t1250: F, t3588: F, t482: F, t1042: F, t3140: F, t460: F, t1242: F, t472: F, t474: F, t3147: F, t479: F, t1248: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3590 = t482 * t3588 * t1250;
    let t3591 = t1042 * t3590;
    let t3594 = t460 * t3140;
    let t3596 = F::cast_from(1.0_f64) / t1242 / t472;
    let t3597 = t3596 * t474;
    let t3598 = t479 * t3147;
    let t3599 = t3597 * t3598;
    let t3600 = t3594 * t3599;
    let t3601 = t1248 * t1248;
    (t3590, t3591, t3594, t3596, t3597, t3598, t3599, t3600, t3601)
}
