//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 525/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk525<F: Float>(t322: F, t1035: F, t1348: F, t2406: F, t2408: F, t2436: F, t2437: F, t2438: F, t2441: F, t352: F, t855: F, t1357: F, t457: F, t898: F) -> (F, F, F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t2445 = t1348 * t1035;
    let t2449 = piecewise5::<f64>(t323, t2406 + t2408, t331, t2436, -F::new(0.21e1) * t2437 * t2438 - F::new(0.105e1) * t855 * t2441 * t352 - F::new(0.1575e1) * t2445 * t2438);
    let t2451 = F::new(4.0) * t1357;
    let t2452 = t898 * t457;
    (t2445, t2449, t2451, t2452)
}
