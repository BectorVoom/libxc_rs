//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 465/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk465<F: Float>(t322: F, t2393: F, t1035: F, t1348: F, t2406: F, t2408: F, t2436: F, t2437: F, t2438: F, t352: F, t855: F, t1357: F, t457: F, t898: F, t41: F, t406: F, t899: F) -> (F, F, F, F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t332 = 0.25e1 < t322;
    let t2441 = piecewise3(t332, t2393, 0.0);
    let t2445 = t1348 * t1035;
    let t2449 = piecewise5(t323, t2406 + t2408, t331, t2436, -0.21e1 * t2437 * t2438 - 0.105e1 * t855 * t2441 * t352 - 0.1575e1 * t2445 * t2438);
    let t2451 = 4.0 * t1357;
    let t2452 = t898 * t457;
    let t2453 = t41 * t2452;
    let t2454 = t406 * t899;
    (t2441, t2445, t2449, t2451, t2452, t2453, t2454)
}
