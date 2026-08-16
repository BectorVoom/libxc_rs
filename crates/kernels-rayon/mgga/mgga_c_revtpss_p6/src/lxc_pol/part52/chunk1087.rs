//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1087/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1087(t27383: f64, t34097: f64, t1468: f64, t1962: f64, t1544: f64, t1583: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t26590: f64, t28460: f64, t32491: f64, t34079: f64, t34090: f64, t7432: f64, t7782: f64, t8657: f64, t892: f64) -> (f64, f64, f64) {
    let t34098 = t27383 * t34097;
    let t34100 = t1468 * t1962;
    let t34126 = t198 * t207 * t34079 * t892 + 3.0_f64 * t1544 * t2403 * t8657 - t1583 * t1940 * t32491 - t1940 * t1962 * t28460 + 2.0_f64 * t1940 * t26590 * t34097 - t1940 * t7432 * t7782 - 3.0_f64 * t2403 * t34090 * t7432;
    (t34098, t34100, t34126)
}
