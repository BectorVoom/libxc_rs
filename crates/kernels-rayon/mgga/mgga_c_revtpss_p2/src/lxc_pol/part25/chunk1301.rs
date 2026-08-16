//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1301/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1301(t33: f64, t265: f64, t502: f64, t94272: f64, t94324: f64, t94213: f64, t10326: f64, t2003: f64, t2258: f64, t25792: f64, t57: f64, t606: f64, t7215: f64, t25082: f64, t49630: f64, t8717: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t94325 = t94272 + t94324;
    let t94326 = piecewise3(t503, 0.0_f64, t94213);
    let t94336 = piecewise3(t400, t94325, t94326 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t25792 * t606 - 3.0_f64 / 2.0_f64 * t7215 * t2258 - t2003 * t10326 / 2.0_f64);
    let t94341 = 9.0_f64 * t25082 * t8717 * t49630;
    (t94336, t94341)
}
