//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2197/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2197<F: Float>(t28019: F, t531: F, t2014: F, t7238: F, t25866: F, t7898: F, t13867: F, t28167: F, t8996: F, t13872: F, t13517: F, t196: F, t197: F) -> (F, F, F, F, F) {
    let t101417 = t531 * t28019;
    let t101420 = F::cast_from(6.0_f64) * t2014 * t101417 * t7238;
    let t101422 = F::cast_from(6.0_f64) * t7898 * t25866;
    let t101428 = F::cast_from(12.0_f64) * t28167 * t8996 * t13867;
    let t101431 = F::cast_from(6.0_f64) * t28167 * t8996 * t13872;
    let t101435 = t13517 * t196 * t197;
    (t101420, t101422, t101428, t101431, t101435)
}
