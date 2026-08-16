//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1105/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1105(t40715: f64, t40719: f64, t47233: f64, t47235: f64, t47238: f64, t47242: f64, t47263: f64, t47265: f64, t47267: f64, t47269: f64, t47271: f64, t47275: f64, t47280: f64, t47287: f64, t47292: f64, t47295: f64, t47302: f64, t534: f64, t72: f64, t9595: f64) -> f64 {
    let t48967 = 0.5107751987195740728e-4_f64 * t47233 + 0.79828278012425390427e-1_f64 * t47235 - 0.2727466165424534173e-1_f64 * t47238 + 0.72732431077987577947e-1_f64 * t47242 + 0.38422568777328955681e-2_f64 * t40715 - 0.17347588262831798123e-3_f64 * t40719 + 0.3405167991463827152e-4_f64 * t47263 + 0.1702583995731913576e-4_f64 * t47265 + 0.638468998399467591e-4_f64 * t47267 + 0.212822999466489197e-4_f64 * t47269 - 0.212822999466489197e-4_f64 * t47271 + 0.23942587439980034662e-4_f64 * t47275 - 0.3405167991463827152e-4_f64 * t47280 + 0.5107751987195740728e-4_f64 * t47287 - 0.5107751987195740728e-4_f64 * t47292 + 2.0_f64 * t72 * t534 * t9595 - 0.5987120850931904282e-1_f64 * t47295 - 0.85129199786595678799e-5_f64 * t47302;
    t48967
}
