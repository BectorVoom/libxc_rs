//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 698/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk698(t30: f64, t265: f64, t393: f64, t1544: f64, t2071: f64, t207: f64, t8019: f64, t1583: f64, t1940: f64, t198: f64, t2403: f64, t7432: f64, t892: f64, t1468: f64, t1469: f64, t2078: f64, t45: f64, t7787: f64, t7991: f64, t8020: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8031 = t2071 * t1544;
    let t8034 = t207 * t8019;
    let t8039 = -t1583 * t1940 * t7432 + t198 * t8034 * t892 + 3.0_f64 * t2403 * t8031;
    let t8040 = piecewise3(t394, 0.0_f64, t8039);
    let t8045 = piecewise3(t120, 3.0_f64 / 2.0_f64 * t2403 * t7991 + t1940 * t8020 * t30 / 2.0_f64 - t1940 * t7432 * t7787 / 2.0_f64 + t1940 * t2071 * t1468 / 2.0_f64, t2078 * t1469 / 2.0_f64 + t8040 * t45 / 2.0_f64);
    (t8039, t8040, t8045)
}
