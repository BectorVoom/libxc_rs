//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 974/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk974(t265: f64, t502: f64, t29154: f64, t29210: f64, t29258: f64, t29311: f64, t3801: f64, t8220: f64, t1298: f64, t1832: f64, t1300: f64, t198: f64, t27037: f64, t27041: f64, t27754: f64, t336: f64, t5023: f64, t5501: f64, t7673: f64) -> f64 {
    let t503 = t265 < t502;
    let t29313 = t29154 + t29210 + t29258 + t29311;
    let t29317 = t8220 * t3801;
    let t29322 = t1832 * t1298;
    let t29329 = piecewise3(t503, t1300 * t198 * t29313 * t336 - t1298 * t29317 * t5023 - t1832 * t27037 * t5023 + 2.0_f64 * t27041 * t29322 * t5023 - t5023 * t5501 * t7673, t27754);
    t29329
}
