//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 937/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk937<F: Float>(t39308: F, t42909: F, t43723: F, t45309: F, t45316: F, t45318: F, t45323: F, t45325: F, t45327: F, t45329: F, t45331: F, t45333: F, t45337: F, t45339: F, t45341: F, t45345: F, t45349: F, t530: F) -> (F,) {
    let t48429 = 0.212822999466489197e-4 * t45309 + 0.638468998399467591e-4 * t45316 + t42909 - 0.39726959900411316773e-4 * t45318 - 0.13242319966803772257e-3 * t39308 - 0.3192344991997337955e-4 * t45323 + 0.5107751987195740728e-4 * t45325 + 0.1702583995731913576e-4 * t45327 - 0.1702583995731913576e-4 * t45329 - 0.11918087970123395032e-3 * t45331 + 0.11918087970123395032e-3 * t45333 - 0.2553875993597870364e-4 * t45337 + 0.2553875993597870364e-4 * t45339 - 0.4726e1 * t530 * t43723 - 0.39726959900411316773e-4 * t45341 - 0.1702583995731913576e-4 * t45345 - 0.85129199786595678799e-5 * t45349;
    (t48429,)
}
