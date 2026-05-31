//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2206/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2206<F: Float>(t1459: F, t28277: F, t28280: F, t5795: F, t7331: F, t28268: F, t116: F, t28042: F, t572: F, t670: F, t2371: F, t28276: F) -> (F, F, F, F, F, F) {
    let t101613 = F::cast_from(12.0_f64) * t1459 * t28277;
    let t101617 = F::cast_from(6.0_f64) * t1459 * t28280;
    let t101619 = F::cast_from(12.0_f64) * t5795 * t7331;
    let t101621 = F::cast_from(12.0_f64) * t1459 * t28268;
    let t101622 = t116 * t28042;
    let t101625 = F::cast_from(12.0_f64) * t572 * t101622 * t670;
    let t101628 = F::cast_from(6.0_f64) * t572 * t28276 * t2371;
    (t101613, t101617, t101619, t101621, t101625, t101628)
}
