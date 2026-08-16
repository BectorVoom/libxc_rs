//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2042/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2042(t33: f64, t265: f64, t502: f64, t110840: f64, t110883: f64, t110920: f64, t110954: f64, t110989: f64, t1469: f64, t18281: f64, t2085: f64, t28578: f64, t30503: f64, t4186: f64, t57: f64, t5825: f64, t606: f64, t7468: f64, t8059: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t110992 = piecewise3(t503, 0.0_f64, t110840);
    let t111004 = piecewise3(t400, t110883 + t110920 + t110954 + t110989, t110992 * t57 / 2.0_f64 - t30503 * t606 / 2.0_f64 - t28578 * t1469 - t8059 * t4186 - t7468 * t5825 / 2.0_f64 - t2085 * t18281 / 2.0_f64);
    t111004
}
