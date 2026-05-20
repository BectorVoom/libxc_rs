//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 708/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk708<F: Float>(t532: F, t7535: F, t1450: F, t2107: F, t7315: F, t118: F, t1310: F, t1453: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t2108: F, t2322: F, t4254: F, t508: F, t569: F, t649: F, t651: F, t671: F, t7235: F, t7357: F, t7359: F, t7367: F, t7374: F, t7378: F, t7474: F, t7484: F, t7489: F) -> (F, F, F, F) {
    let t7536 = t532 * t7535;
    let t7537 = t7536 * t1450;
    let t7539 = t2107 * t7315;
    let t7541 = -t118 * t7474 - t1310 * t2052 + t1453 * t2093 + F::new(3.0) * t2014 * t7489 + t2014 * t7537 - t2014 * t7539 - F::new(2.0) * t2056 * t2322 - F::new(2.0) * t2056 * t4254 - t2089 * t649 + t2108 * t7235 - t508 * t7357 + t569 * t7484 - F::new(2.0) * t651 * t7367 - F::new(2.0) * t651 * t7374 - F::new(2.0) * t651 * t7378 - F::new(2.0) * t671 * t7359;
    (t7536, t7537, t7539, t7541)
}
