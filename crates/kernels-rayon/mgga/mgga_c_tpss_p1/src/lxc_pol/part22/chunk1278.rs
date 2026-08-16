//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1278/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1278(t30: f64, t259: f64, t379: f64, t20576: f64, t1289: f64, t1819: f64, t20545: f64, t3431: f64, t45: f64, t581: f64, t5870: f64, t6374: f64, t1006: f64, t1497: f64, t1692: f64, t1812: f64, t18728: f64, t18807: f64, t20012: f64, t20018: f64, t20021: f64, t20025: f64, t20041: f64, t20048: f64, t20050: f64, t20054: f64, t20058: f64, t20065: f64, t20417: f64, t20510: f64, t20514: f64, t20526: f64, t20544: f64, t2439: f64, t33: f64, t5671: f64, t5678: f64, t5849: f64, t5853: f64, t6207: f64, t6214: f64, t6354: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t20577 = piecewise3(t380, 0.0_f64, t20576);
    let t20584 = piecewise3(t120, t20545, t5870 * t1289 / 2.0_f64 + t1819 * t3431 / 2.0_f64 + t20577 * t45 / 2.0_f64 + t6374 * t581 / 2.0_f64);
    let t20631 = 3.0_f64 * t20417 * t20012 + 3.0_f64 / 2.0_f64 * t2439 * t5849 * t6207 - 3.0_f64 / 2.0_f64 * t18728 * t20018 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t20021 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t20025 + 3.0_f64 / 2.0_f64 * t2439 * t6354 * t5671 + t1692 * t20510 * t33 / 2.0_f64 - t1692 * t20514 * t5678 / 2.0_f64 + t1692 * t6354 * t1006 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t18728 * t20041 - t1692 * t18807 * t6214 / 2.0_f64 + t20526 * t20048 - t1692 * t5853 * t20050 / 2.0_f64 - t1692 * t5853 * t20054 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t20058 + t1692 * t5849 * t1497 / 2.0_f64 - t1692 * t5853 * t20065 / 2.0_f64 - t20544;
    (t20577, t20584, t20631)
}
