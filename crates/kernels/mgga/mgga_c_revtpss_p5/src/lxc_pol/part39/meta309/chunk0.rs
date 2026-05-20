//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1078/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1078<F: Float>(t2439: F, t931: F, t2915: F, t698: F, t2922: F, t913: F, t275: F, t290: F, t2925: F, t2935: F, t945: F, t2967: F, t941: F) -> (F, F, F, F, F, F) {
    let t11366 = t2439 * t931;
    let t11368 = t698 * t2915;
    let t11384 = F::new(1.0) / t2922 / t913;
    let t11385 = t275 * t11384;
    let t11387 = F::new(1.0) / t2925 / t290;
    let t11399 = t2935 * t945;
    let t11404 = t941 * t2967;
    (t11366, t11368, t11385, t11387, t11399, t11404)
}
