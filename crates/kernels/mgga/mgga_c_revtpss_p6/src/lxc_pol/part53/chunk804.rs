//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 804/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk804<F: Float>(t532: F, t8598: F, t4147: F, t2014: F, t2040: F, t2042: F, t116: F, t8453: F, t572: F, t117: F, t8460: F, t136: F, t8440: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8599 = t532 * t8598;
    let t8600 = t8599 * t4147;
    let t8601 = t2014 * t8600;
    let t8609 = t2040 * t2042;
    let t8611 = t116 * t8453;
    let t8613 = F::cast_from(6.0_f64) * t572 * t8611;
    let t8614 = t117 * t8460;
    let t8616 = F::cast_from(3.0_f64) * t572 * t8614;
    let t8621 = t136 * t8440;
    (t8599, t8600, t8601, t8609, t8611, t8613, t8614, t8616, t8621)
}
