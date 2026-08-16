//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1724/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1724(t87125: f64, t6587: f64, t20292: f64, t5825: f64, t12305: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t89780 = -t87125;
    let t89808 = t6587 * t6587;
    let t89822 = t20292 * t5825;
    let t89824 = t128 * t12305 * t89822;
    (t89780, t89808, t89822, t89824)
}
