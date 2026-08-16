//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 868/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk868(t30: f64, t259: f64, t379: f64, t1364: f64, t1812: f64, t207: f64, t6353: f64, t1398: f64, t1692: f64, t198: f64, t2439: f64, t5853: f64, t823: f64, t1288: f64, t1289: f64, t1819: f64, t45: f64, t6153: f64, t6331: f64, t6354: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t6365 = t1812 * t1364;
    let t6368 = t207 * t6353;
    let t6373 = -t1398 * t1692 * t5853 + t198 * t6368 * t823 + 3.0_f64 * t2439 * t6365;
    let t6374 = piecewise3(t380, 0.0_f64, t6373);
    let t6379 = piecewise3(t120, 3.0_f64 / 2.0_f64 * t2439 * t6331 + t1692 * t6354 * t30 / 2.0_f64 - t1692 * t5853 * t6153 / 2.0_f64 + t1692 * t1812 * t1288 / 2.0_f64, t1819 * t1289 / 2.0_f64 + t6374 * t45 / 2.0_f64);
    (t6365, t6373, t6374, t6379)
}
