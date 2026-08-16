//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1236/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1236(t30: f64, t265: f64, t393: f64, t1518: f64, t2163: f64, t7855: f64, t1469: f64, t2129: f64, t45: f64, t7794: f64, t1479: f64, t343: f64, t136: f64, t1785: f64, t2138: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8158 = t2163 * t1518;
    let t8161 = piecewise3(t394, 0.0_f64, t7855);
    let t8166 = piecewise3(t120, t7794, t2129 * t1469 / 2.0_f64 + t8161 * t45 / 2.0_f64);
    let t8171 = t1479 * t343;
    let t8172 = t8171 * t136;
    let t8177 = t1785 * t2138;
    (t8158, t8161, t8166, t8171, t8172, t8177)
}
