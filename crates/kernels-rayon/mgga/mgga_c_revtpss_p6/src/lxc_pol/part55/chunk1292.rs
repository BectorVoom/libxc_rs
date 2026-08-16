//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1292/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1292(t128903: f64, t128904: f64, t128906: f64, t128910: f64, t128917: f64, t1843: f64, t1911: f64, t32107: f64, t32109: f64, t32112: f64, t33286: f64, t33296: f64, t34399: f64, t5517: f64, t7489: f64, t7539: f64, t8463: f64, t8886: f64) -> f64 {
    let t131045 = -t1843 * t33286 + t1911 * t33296 + 3.0_f64 * t34399 * t7489 - t34399 * t7539 - t5517 * t8886 + t128903 - t128904 + t128906 - t128910 - t128917 - t32107 - t32109 - t32112 - t8463;
    t131045
}
