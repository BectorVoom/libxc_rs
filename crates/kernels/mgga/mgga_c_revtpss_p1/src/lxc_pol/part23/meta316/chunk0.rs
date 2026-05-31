//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1604/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1604<F: Float>(t1466: F, t2246: F, t2275: F, t4186: F, t580: F, t9342: F, t2282: F, t10389: F, t1469: F, t2299: F, t10398: F, t2306: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13272 = t1466 * t2246;
    let t13302 = t2275 * t4186;
    let t13309 = F::cast_from(2.0_f64) * t580;
    let t13310 = F::cast_from(6.0_f64) * t9342;
    let t13324 = t2282 * t4186;
    let t13368 = t10389 * t1469;
    let t13371 = t2299 * t4186;
    let t13378 = t10398 * t1469;
    let t13381 = t2306 * t4186;
    (t13272, t13302, t13309, t13310, t13324, t13368, t13371, t13378, t13381)
}
