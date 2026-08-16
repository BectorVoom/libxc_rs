//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 793/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk793(t33: f64, t265: f64, t502: f64, t1940: f64, t8490: f64, t8494: f64, t8542: f64, t57: f64, t1936: f64, t6985: f64, t8453: f64, t93: f64, t1312: f64, t8460: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8552 = t1940 * t8490 * t33 / 2.0_f64 - t1940 * t8494 * t33 / 2.0_f64;
    let t8553 = piecewise3(t503, 0.0_f64, t8542);
    let t8556 = piecewise3(t400, t8552, t8553 * t57 / 2.0_f64);
    let t8559 = t6985 * t1936;
    let t8562 = 2.0_f64 * t93 * t8453;
    let t8563 = t1312 * t8460;
    (t8553, t8556, t8559, t8562, t8563)
}
