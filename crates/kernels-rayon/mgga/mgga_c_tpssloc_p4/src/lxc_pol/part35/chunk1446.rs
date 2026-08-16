//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1446/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1446(t104018: f64, t104085: f64, t104088: f64, t104094: f64, t104303: f64, t104387: f64, t104413: f64, t1737: f64, t1748: f64, t22197: f64, t22243: f64, t22246: f64, t22275: f64, t24733: f64, t27684: f64, t27711: f64, t29594: f64, t29644: f64, t29648: f64, t475: f64, t68: f64, t7326: f64, t7328: f64, t7339: f64, t7345: f64, t8040: f64, t95327: f64, t95335: f64) -> f64 {
    let t109493 = t7339 * t22246 / 1536.0_f64 - t104018 * t1748 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t7345 * t22197 - t24733 * t22275 / 512.0_f64 - t104085 / 288.0_f64 - t104088 / 144.0_f64 - t104094 / 576.0_f64 - 0.24223653656484234513e-2_f64 * t27711 * t29594 - 0.30279567070605293142e-3_f64 * t27684 * t29594 + 0.10093189023535097714e-3_f64 * t7326 * t7328 * t22243 * t68 * t475 - 0.30279567070605293142e-3_f64 * t104387 * t8040 - t95335 / 2304.0_f64 + 0.48447307312968469026e-2_f64 * t104413 * t8040 - 0.48447307312968469026e-2_f64 * t95327 * t29644 + 0.24223653656484234513e-2_f64 * t95327 * t29648 - t104303 * t1737 / 48.0_f64;
    t109493
}
