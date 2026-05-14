//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 964/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk964<F: Float>(t40715: F, t40719: F, t47233: F, t47235: F, t47238: F, t47242: F, t47263: F, t47265: F, t47267: F, t47269: F, t47271: F, t47275: F, t47280: F, t47287: F, t47292: F, t47295: F, t47302: F, t534: F, t72: F, t9595: F) -> (F,) {
    let t48967 = 0.5107751987195740728e-4 * t47233 + 0.79828278012425390427e-1 * t47235 - 0.2727466165424534173e-1 * t47238 + 0.72732431077987577947e-1 * t47242 + 0.38422568777328955681e-2 * t40715 - 0.17347588262831798123e-3 * t40719 + 0.3405167991463827152e-4 * t47263 + 0.1702583995731913576e-4 * t47265 + 0.638468998399467591e-4 * t47267 + 0.212822999466489197e-4 * t47269 - 0.212822999466489197e-4 * t47271 + 0.23942587439980034662e-4 * t47275 - 0.3405167991463827152e-4 * t47280 + 0.5107751987195740728e-4 * t47287 - 0.5107751987195740728e-4 * t47292 + 2.0 * t72 * t534 * t9595 - 0.5987120850931904282e-1 * t47295 - 0.85129199786595678799e-5 * t47302;
    (t48967,)
}
