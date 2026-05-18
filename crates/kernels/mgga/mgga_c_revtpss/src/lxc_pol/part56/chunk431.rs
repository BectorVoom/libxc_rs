//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 431/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk431<F: Float>(t251: F, t785: F, t780: F, t2439: F, t212: F, t860: F, t689: F, t779: F, t887: F, t211: F, t784: F, t209: F) -> (F, F, F, F, F) {
    let t2440 = t785 * t251;
    let t2441 = t2440 * t780;
    let t2443 = F::new(0.65049603595885220126e-3) * t2439 * t2441;
    let t2444 = t212 * t860;
    let t2445 = t2444 * t780;
    let t2446 = t689 * t2445;
    let t2448 = t779 * t887;
    let t2449 = t689 * t2448;
    let t2452 = F::new(1.0) / t784 / t211;
    let t2453 = t209 * t2452;
    (t2443, t2446, t2449, t2452, t2453)
}
