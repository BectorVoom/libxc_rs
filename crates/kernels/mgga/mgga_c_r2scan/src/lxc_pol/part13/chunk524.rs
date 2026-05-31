//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 524/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk524<F: Float>(t322: F, t1035: F, t1348: F, t2406: F, t2408: F, t2436: F, t2437: F, t2438: F, t2441: F, t352: F, t855: F, t1357: F, t457: F, t898: F) -> (F, F, F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t2445 = t1348 * t1035;
    let t2449 = piecewise5::<F>(t323, t2406 + t2408, t331, t2436, -F::cast_from(0.21e1_f64) * t2437 * t2438 - F::cast_from(0.105e1_f64) * t855 * t2441 * t352 - F::cast_from(0.1575e1_f64) * t2445 * t2438);
    let t2451 = F::cast_from(4.0_f64) * t1357;
    let t2452 = t898 * t457;
    (t2445, t2449, t2451, t2452)
}
