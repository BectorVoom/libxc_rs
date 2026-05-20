//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2273/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2273<F: Float>(t2167: F, t6951: F, t1913: F, t8249: F, t105792: F, t105794: F, t105796: F, t105798: F, t105800: F, t105802: F, t105804: F, t113039: F, t113050: F, t1458: F, t1914: F, t2168: F, t2172: F, t22533: F, t22571: F, t29490: F) -> F {
    let t113053 = t2167 * t6951;
    let t113054 = t1913 * t8249;
    let t113060 = t105792 + t1458 * (t113039 + t113050) + t113053 + F::new(2.0) * t113054 + F::new(2.0) * t1914 * t29490 + t105794 + t105796 + t22533 * t2172 + t105798 + t2168 * t22571 + t105800 + t105802 + t105804;
    t113060
}
