//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 772/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk772(t33: f64, t259: f64, t479: f64, t4818: f64, t5305: f64, t1289: f64, t1402: f64, t1497: f64, t1594: f64, t4579: f64, t481: f64, t5059: f64, t57: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t5306 = piecewise3(t480, t5305, t4818);
    let t5313 = piecewise3(t386, t4818 * t33 / 2.0_f64 + t1402 * t1497 + t259 * t5059 / 2.0_f64, t5306 * t57 / 2.0_f64 - t1594 * t1289 - t481 * t4579 / 2.0_f64);
    (t5306, t5313)
}
