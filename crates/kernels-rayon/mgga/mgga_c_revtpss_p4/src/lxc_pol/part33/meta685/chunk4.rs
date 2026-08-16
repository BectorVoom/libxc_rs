//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2267/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2267(t33: f64, t265: f64, t502: f64, t107868: f64, t112989: f64, t108049: f64, t1469: f64, t18281: f64, t2159: f64, t29329: f64, t30936: f64, t4186: f64, t57: f64, t5825: f64, t606: f64, t7677: f64, t8227: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t112990 = piecewise3(t503, t112989, t107868);
    let t113002 = piecewise3(t400, t108049, t112990 * t57 / 2.0_f64 - t30936 * t606 / 2.0_f64 - t29329 * t1469 - t8227 * t4186 - t7677 * t5825 / 2.0_f64 - t2159 * t18281 / 2.0_f64);
    t113002
}
