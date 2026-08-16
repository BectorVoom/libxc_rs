//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 739/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk739(t1940: f64, t30: f64, t8490: f64, t8494: f64, t207: f64, t8489: f64, t8493: f64, t198: f64, t2411: f64, t892: f64, t33: f64, t8453: f64, t93: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8498 = t1940 * t8490 * t30 / 2.0_f64 - t1940 * t8494 * t30 / 2.0_f64;
    let t8536 = t207 * t8489;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    let t8552 = t1940 * t8490 * t33 / 2.0_f64 - t1940 * t8494 * t33 / 2.0_f64;
    let t8562 = 2.0_f64 * t93 * t8453;
    (t8498, t8536, t8539, t8542, t8552, t8562)
}
