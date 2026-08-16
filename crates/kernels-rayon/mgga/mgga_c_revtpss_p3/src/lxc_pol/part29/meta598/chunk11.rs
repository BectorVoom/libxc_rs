//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2038/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2038(t33: f64, t265: f64, t502: f64, t103707: f64, t103750: f64, t103778: f64, t103817: f64, t103853: f64, t13312: f64, t1469: f64, t2085: f64, t2258: f64, t26666: f64, t28578: f64, t4186: f64, t57: f64, t606: f64, t7468: f64, t8059: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t103856 = piecewise3(t503, 0.0_f64, t103707);
    let t103868 = piecewise3(t400, t103750 + t103778 + t103817 + t103853, t103856 * t57 / 2.0_f64 - t28578 * t606 - t8059 * t2258 / 2.0_f64 - t26666 * t1469 / 2.0_f64 - t7468 * t4186 - t2085 * t13312 / 2.0_f64);
    t103868
}
