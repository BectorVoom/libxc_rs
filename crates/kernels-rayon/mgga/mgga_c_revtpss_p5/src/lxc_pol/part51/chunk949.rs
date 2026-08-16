//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 949/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk949(t33: f64, t265: f64, t502: f64, t32058: f64, t32088: f64, t57: f64, t606: f64, t8553: f64, t32064: f64, t531: f64, t8594: f64, t7238: f64, t2014: f64, t7235: f64, t8600: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t32089 = piecewise3(t503, 0.0_f64, t32058);
    let t32094 = piecewise3(t400, t32088, t32089 * t57 / 2.0_f64 - t8553 * t606 / 2.0_f64);
    let t32095 = t32064 + t32094;
    let t32098 = t531 * t8594;
    let t32099 = t32098 * t7238;
    let t32101 = 3.0_f64 * t2014 * t32099;
    let t32102 = t7235 * t8600;
    (t32089, t32095, t32098, t32099, t32101, t32102)
}
