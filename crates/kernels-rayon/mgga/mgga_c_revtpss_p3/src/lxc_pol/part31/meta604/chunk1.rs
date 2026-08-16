//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2040/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2040(t25895: f64, t98028: f64, t1892: f64, t7063: f64, t25877: f64, t25881: f64, t1955: f64, t97960: f64, t213: f64, t27960: f64, t27902: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98029 = t25895 * t98028;
    let t98040 = t7063 * t1892;
    let t98041 = t98040 * t25877;
    let t98043 = 0.51405703062096148812e-1_f64 * t98041 * t25881;
    let t98050 = t1955 * t97960;
    let t98056 = t213 * t27960;
    let t98067 = t27902 * t72 * t686;
    (t98029, t98040, t98041, t98043, t98050, t98056, t98067)
}
