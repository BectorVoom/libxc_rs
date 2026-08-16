//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 611/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk611(t3670: f64, t480: f64, t221: f64, t462: f64, t696: f64, t461: f64, t1224: f64, t3367: f64, t1121: f64, t404: f64) -> (f64, f64, f64, f64, f64) {
    let t3671 = t3670 * t480;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / 432.0_f64;
    let t3692 = t1224 * t3367;
    let t3698 = 1.0_f64 / t404 / t1121;
    (t3671, t3682, t3684, t3692, t3698)
}
