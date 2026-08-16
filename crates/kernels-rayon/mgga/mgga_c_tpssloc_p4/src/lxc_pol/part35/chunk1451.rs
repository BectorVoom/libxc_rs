//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1451/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1451(t104231: f64, t104355: f64, t104364: f64, t104367: f64, t104369: f64, t104371: f64, t104375: f64, t2121: f64, t2134: f64, t2136: f64, t2139: f64, t21745: f64, t22038: f64, t22133: f64, t22173: f64, t27599: f64, t28525: f64, t29563: f64, t29615: f64, t460: f64, t471: f64, t488: f64, t4899: f64, t6221: f64, t7310: f64, t7320: f64, t8027: f64, t8028: f64, t8031: f64, t8035: f64, t8040: f64) -> f64 {
    let t109661 = 0.24223653656484234513e-2_f64 * t8028 * t29615 - 0.10093189023535097714e-3_f64 * t2134 * t22038 * t460 * t7320 + t2121 * t4899 * t21745 / 72.0_f64 - t7310 * t22133 / 48.0_f64 - 0.21801288290835811062e-1_f64 * t29563 * t8035 + 0.30279567070605293142e-3_f64 * t8031 * t29615 + 0.24223653656484234513e-2_f64 * t8027 * t28525 * t2136 + 0.30279567070605293142e-3_f64 * t104355 - 0.60559134141210586284e-3_f64 * t104364 - 0.30279567070605293142e-3_f64 * t104367 - t104369 / 1152.0_f64 - t104371 / 576.0_f64 - t104375 / 576.0_f64 - 209.0_f64 / 1296.0_f64 * t471 * t2139 * t22173 * t488 + 0.48447307312968469026e-2_f64 * t104231 * t8040 - t27599 * t6221 / 96.0_f64;
    t109661
}
