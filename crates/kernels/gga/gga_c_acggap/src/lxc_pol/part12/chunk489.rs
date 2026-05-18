//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 489/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk489<F: Float>(t2147: F, t2394: F, t157: F, t2152: F, t524: F, t633: F, t159: F, t2385: F, t619: F, t119: F, t2146: F, t2175: F, t2178: F, t2222: F, t2228: F, t2232: F, t2338: F, t2387: F, t557: F, t616: F, t639: F) -> (F, F, F, F) {
    let t2395 = t2147 * t2394;
    let t2400 = t2152 * t633 * t524 * t157;
    let t2404 = t619 * t159 * t2385;
    let t2407 = t2175 - t2178 + F::new(0.65854491829355115987e0) * t119 * t2387 - F::new(0.65854491829355115987e0) * t2222 * t557 - t2228 + t2232 - F::new(0.4336814094102599731e0) * t2338 * t639 + F::new(0.8673628188205199462e0) * t2146 * t2395 + F::new(0.4336814094102599731e0) * t2146 * t2400 - F::new(0.4336814094102599731e0) * t616 * t2404;
    (t2395, t2400, t2404, t2407)
}
