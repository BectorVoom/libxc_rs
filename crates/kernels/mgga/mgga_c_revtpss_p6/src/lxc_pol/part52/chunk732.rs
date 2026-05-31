//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 732/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk732<F: Float>(t670: F, t7553: F, t117: F, t7373: F, t1459: F, t1461: F, t2113: F, t2115: F, t572: F, t573: F, t7547: F, t38: F, t4173: F) -> (F, F, F, F) {
    let t7554 = t7553 * t670;
    let t7557 = t117 * t7373;
    let t7560 = F::cast_from(3.0_f64) * t1459 * t2115 + F::cast_from(3.0_f64) * t1461 * t2113 + F::cast_from(6.0_f64) * t572 * t7554 + F::cast_from(3.0_f64) * t572 * t7557 + t573 * t7547;
    let t7702 = t4173 * t38;
    (t7554, t7557, t7560, t7702)
}
