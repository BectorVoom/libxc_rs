//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1452/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1452(t104410: f64, t104425: f64, t104435: f64, t104441: f64, t104445: f64, t106348: f64, t1726: f64, t1730: f64, t2132: f64, t2136: f64, t22129: f64, t22137: f64, t27674: f64, t29562: f64, t29600: f64, t29625: f64, t29651: f64, t488: f64, t6178: f64, t6184: f64, t6188: f64, t7310: f64, t7573: f64, t8028: f64, t8031: f64, t8035: f64, t95550: f64) -> f64 {
    let t109694 = t95550 / 3456.0_f64 + 0.30279567070605293142e-3_f64 * t8031 * t29625 - 11.0_f64 / 108.0_f64 * t104410 * t1726 + t27674 * t6184 / 36.0_f64 + t27674 * t6188 / 18.0_f64 - t7310 * t22129 / 288.0_f64 - 0.21801288290835811062e-1_f64 * t29562 * t7573 * t2136 + 0.30279567070605293142e-3_f64 * t29651 * t8035 + 0.24223653656484234513e-2_f64 * t8028 * t29625 + 19.0_f64 / 288.0_f64 * t1730 * t29600 * t488 - 0.10093189023535097714e-3_f64 * t2132 * t106348 * t2136 - t27674 * t6178 / 27.0_f64 + t7310 * t22137 / 36.0_f64 - 0.48447307312968469026e-2_f64 * t104425 + 5.0_f64 / 3456.0_f64 * t104435 + t104441 / 216.0_f64 + t104445 / 768.0_f64;
    t109694
}
