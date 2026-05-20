//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2218/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2218<F: Float>(t108145: F, t108172: F, t108213: F, t108233: F, t108270: F, t108310: F, t108327: F, t108349: F, t108374: F, t108399: F, t108425: F, t108443: F, t108471: F, t108500: F, t108651: F, t108674: F, t1450: F, t2014: F, t532: F) -> F {
    let t108681 = t2014 * t532 * (t108145 + t108172 + t108213 + t108233 + t108270 + t108310 + t108327 + t108349 + t108374 + t108399 + t108425 + t108443 + t108471 + t108500 + t108651 + t108674) * t1450;
    t108681
}
