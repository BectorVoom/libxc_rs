//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1974/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1974(t33: f64, t1469: f64, t2159: f64, t29977: f64, t30936: f64, t57: f64, t5825: f64, t8227: f64, t30734: f64, t118: f64, t1502: f64, t1843: f64, t1911: f64, t2127: f64, t2163: f64, t29497: f64, t29501: f64, t29504: f64, t29507: f64, t29510: f64, t29512: f64, t29578: f64, t29580: f64, t29582: f64, t29585: f64, t30716: f64, t30724: f64, t508: f64, t5877: f64, t5884: f64, t6765: f64, t8152: f64, t8233: f64, t8237: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t30943 = piecewise3(t400, t29977, t30936 * t57 / 2.0_f64 - t8227 * t1469 - t2159 * t5825 / 2.0_f64);
    let t30944 = t30734 + t30943;
    let t30950 = -t118 * t30944 - 2.0_f64 * t1502 * t8233 - 2.0_f64 * t1843 * t8152 + 2.0_f64 * t1911 * t8237 - t2127 * t6765 - t2163 * t5877 - 2.0_f64 * t2163 * t5884 - t30716 * t508 - 2.0_f64 * t30724 * t508 + t29497 + t29501 - t29504 + t29507 - t29510 - t29512 + t29578 + t29580 - t29582 + t29585;
    (t30944, t30950)
}
