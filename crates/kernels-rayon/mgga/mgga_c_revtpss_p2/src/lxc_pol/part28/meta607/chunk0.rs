//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2104/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2104(t1883: f64, t4077: f64, t27902: f64, t686: f64, t72: f64, t25878: f64, t97732: f64, t27840: f64, t689: f64, t94674: f64, t94669: f64, t26069: f64, t97922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98062 = t1883 * t4077;
    let t98067 = t27902 * t72 * t686;
    let t98069 = 0.51405703062096148812e-1_f64 * t25878 * t98067;
    let t98071 = 0.51405703062096148812e-1_f64 * t25878 * t97732;
    let t98077 = t27840 * t689;
    let t98078 = t94674 * t98077;
    let t98081 = 0.15421710918628844644e0_f64 * t94669 * t98077;
    let t98084 = t26069 * t97922;
    (t98062, t98067, t98069, t98071, t98078, t98081, t98084)
}
