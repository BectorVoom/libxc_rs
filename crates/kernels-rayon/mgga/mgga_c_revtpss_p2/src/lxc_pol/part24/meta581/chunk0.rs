//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1805/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1805(t1300: f64, t198: f64, t336: f64, t89888: f64, t89930: f64, t90293: f64, t90321: f64, t90323: f64, t90327: f64, t90329: f64, t90332: f64, t90336: f64, t90339: f64, t90341: f64, t90343: f64, t90346: f64, t90349: f64, t91440: f64, t91748: f64) -> f64 {
    let t91754 = t198 * t336 * (t89888 + t89930 + t91440 + t91748) * t1300 + t90293 + t90321 - t90323 + t90327 + t90329 - t90332 - t90336 + t90339 + t90341 + t90343 + t90346 - t90349;
    t91754
}
