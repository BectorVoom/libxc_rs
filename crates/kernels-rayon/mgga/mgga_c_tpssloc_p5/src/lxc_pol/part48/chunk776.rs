//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 776/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk776(t24260: f64, t24280: f64, t858: f64, t23230: f64, t225: f64, t7072: f64, t23198: f64, t23206: f64, t23209: f64, t23220: f64, t23224: f64, t23232: f64, t23235: f64, t23239: f64, t24200: f64, t24235: f64, t24237: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t7087: f64, t7092: f64, t7107: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64) {
    let t24281 = t24260 + t24280;
    let t24282 = t858 * t24281;
    let t24291 = 0.16449340668482264365e-1_f64 * t23230;
    let t24297 = t7072 * t225;
    let t24300 = t24200 * t259 + t24235 * t259 + 2.0_f64 * t24237 * t259 - 2.0_f64 * t2713 * t7107 - t855 * t24282 + 0.3289868133696452873e-1_f64 * t23198 + 0.3289868133696452873e-1_f64 * t23206 + 0.16449340668482264365e-1_f64 * t23209 - 0.16449340668482264365e-1_f64 * t23220 - 0.3289868133696452873e-1_f64 * t23224 + 2.0_f64 * t7087 * t2720 - t24291 + 0.15352717957250113407e0_f64 * t23232 + 0.76763589786250567036e-1_f64 * t23235 - 0.6579736267392905746e-1_f64 * t23239 + 4.0_f64 * t2597 * t7092 - 2.0_f64 * t24297 * t866;
    (t24281, t24282, t24297, t24300)
}
