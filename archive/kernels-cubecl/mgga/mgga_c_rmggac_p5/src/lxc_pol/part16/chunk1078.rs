//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1078/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1078<F: Float>(t39308: F, t42909: F, t43723: F, t45309: F, t45316: F, t45318: F, t45323: F, t45325: F, t45327: F, t45329: F, t45331: F, t45333: F, t45337: F, t45339: F, t45341: F, t45345: F, t45349: F, t530: F) -> F {
    let t48429 = F::cast_from(0.212822999466489197e-4_f64) * t45309 + F::cast_from(0.638468998399467591e-4_f64) * t45316 + t42909 - F::cast_from(0.39726959900411316773e-4_f64) * t45318 - F::cast_from(0.13242319966803772257e-3_f64) * t39308 - F::cast_from(0.3192344991997337955e-4_f64) * t45323 + F::cast_from(0.5107751987195740728e-4_f64) * t45325 + F::cast_from(0.1702583995731913576e-4_f64) * t45327 - F::cast_from(0.1702583995731913576e-4_f64) * t45329 - F::cast_from(0.11918087970123395032e-3_f64) * t45331 + F::cast_from(0.11918087970123395032e-3_f64) * t45333 - F::cast_from(0.2553875993597870364e-4_f64) * t45337 + F::cast_from(0.2553875993597870364e-4_f64) * t45339 - F::cast_from(0.4726e1_f64) * t530 * t43723 - F::cast_from(0.39726959900411316773e-4_f64) * t45341 - F::cast_from(0.1702583995731913576e-4_f64) * t45345 - F::cast_from(0.85129199786595678799e-5_f64) * t45349;
    t48429
}
