//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1078/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1078(t39308: f64, t42909: f64, t43723: f64, t45309: f64, t45316: f64, t45318: f64, t45323: f64, t45325: f64, t45327: f64, t45329: f64, t45331: f64, t45333: f64, t45337: f64, t45339: f64, t45341: f64, t45345: f64, t45349: f64, t530: f64) -> f64 {
    let t48429 = 0.212822999466489197e-4_f64 * t45309 + 0.638468998399467591e-4_f64 * t45316 + t42909 - 0.39726959900411316773e-4_f64 * t45318 - 0.13242319966803772257e-3_f64 * t39308 - 0.3192344991997337955e-4_f64 * t45323 + 0.5107751987195740728e-4_f64 * t45325 + 0.1702583995731913576e-4_f64 * t45327 - 0.1702583995731913576e-4_f64 * t45329 - 0.11918087970123395032e-3_f64 * t45331 + 0.11918087970123395032e-3_f64 * t45333 - 0.2553875993597870364e-4_f64 * t45337 + 0.2553875993597870364e-4_f64 * t45339 - 0.4726e1_f64 * t530 * t43723 - 0.39726959900411316773e-4_f64 * t45341 - 0.1702583995731913576e-4_f64 * t45345 - 0.85129199786595678799e-5_f64 * t45349;
    t48429
}
