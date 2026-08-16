//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1052/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1052(t30: f64, t265: f64, t393: f64, t32534: f64, t1940: f64, t2403: f64, t26425: f64, t26585: f64, t28472: f64, t31873: f64, t32487: f64, t32491: f64, t32499: f64, t32506: f64, t32508: f64, t45: f64, t605: f64, t606: f64, t7010: f64, t7092: f64, t7432: f64, t8657: f64, t8660: f64, t8671: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t32535 = piecewise3(t394, 0.0_f64, t32534);
    let t32540 = piecewise3(t120, 3.0_f64 / 2.0_f64 * t2403 * t8657 * t7010 + t1940 * t32487 * t30 / 2.0_f64 - t1940 * t32491 * t7092 / 2.0_f64 + t1940 * t8657 * t605 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t32499 - t1940 * t26585 * t8660 / 2.0_f64 + t28472 * t32506 - t1940 * t7432 * t32508 / 2.0_f64 - t1940 * t7432 * t31873 / 2.0_f64, t32535 * t45 / 2.0_f64 + t8671 * t606 / 2.0_f64);
    (t32535, t32540)
}
