//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1960/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1960<F: Float>(t30191: F, t572: F, t117: F, t30004: F, t1469: F, t25137: F, t26776: F, t29355: F, t5819: F, t5825: F, t5842: F, t61: F, t7571: F) -> (F, F, F, F) {
    let t30193 = F::cast_from(6.0_f64) * t572 * t30191;
    let t30194 = t117 * t30004;
    let t30196 = F::cast_from(3.0_f64) * t572 * t30194;
    let t30681 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t5842 * t61 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t29355 * t1469 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t26776 * t5819 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7571 * t5825 - t25137;
    (t30193, t30194, t30196, t30681)
}
