//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 469/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk469<F: Float>(t2548: F, t730: F, t722: F, t164: F, t172: F, t2538: F, t123: F, t147: F, t2434: F) -> (F, F, F, F, F, F, F, F) {
    let t2549 = t2548 * t730;
    let t2552 = t722 * t722;
    let t2553 = 1.0 / t2552;
    let t2554 = t164 * t2553;
    let t2555 = t172 * t172;
    let t2556 = 1.0 / t2555;
    let t2557 = t2538 * t2556;
    let t2562 = 0.14764627977777777777e-2 * t123 * t2434 * t147;
    (t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562)
}
