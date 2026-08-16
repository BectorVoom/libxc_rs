//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2048/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2048(t25878: f64, t98067: f64, t97732: f64, t27840: f64, t689: f64, t94674: f64, t94669: f64, t26069: f64, t97922: f64, t28011: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98069 = 0.51405703062096148812e-1_f64 * t25878 * t98067;
    let t98071 = 0.51405703062096148812e-1_f64 * t25878 * t97732;
    let t98077 = t27840 * t689;
    let t98078 = t94674 * t98077;
    let t98081 = 0.15421710918628844644e0_f64 * t94669 * t98077;
    let t98084 = t26069 * t97922;
    let t98087 = t28011 * t72 * t686;
    (t98069, t98071, t98078, t98081, t98084, t98087)
}
