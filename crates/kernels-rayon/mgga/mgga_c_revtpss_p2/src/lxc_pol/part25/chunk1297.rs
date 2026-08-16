//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1297/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1297(t30: f64, t10326: f64, t1996: f64, t2258: f64, t25744: f64, t45: f64, t606: f64, t7194: f64, t93409: f64, t94214: f64, t25759: f64, t51806: f64, t27799: f64, t50066: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t94224 = piecewise3(t120, t93409, t94214 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t25744 * t606 + 3.0_f64 / 2.0_f64 * t7194 * t2258 + t1996 * t10326 / 2.0_f64);
    let t94228 = t25759 * t51806;
    let t94231 = t27799 * t50066;
    (t94224, t94228, t94231)
}
