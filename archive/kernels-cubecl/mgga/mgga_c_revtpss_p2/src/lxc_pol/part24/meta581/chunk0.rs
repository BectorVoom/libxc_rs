//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1805/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1805<F: Float>(t1300: F, t198: F, t336: F, t89888: F, t89930: F, t90293: F, t90321: F, t90323: F, t90327: F, t90329: F, t90332: F, t90336: F, t90339: F, t90341: F, t90343: F, t90346: F, t90349: F, t91440: F, t91748: F) -> F {
    let t91754 = t198 * t336 * (t89888 + t89930 + t91440 + t91748) * t1300 + t90293 + t90321 - t90323 + t90327 + t90329 - t90332 - t90336 + t90339 + t90341 + t90343 + t90346 - t90349;
    t91754
}
