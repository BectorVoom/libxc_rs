//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 939/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk939<F: Float>(t39370: F, t39388: F, t45394: F, t45396: F, t45403: F, t45407: F, t45411: F, t45415: F, t45420: F, t45424: F, t45428: F, t45432: F, t45436: F, t45439: F, t45441: F, t45446: F, t45451: F) -> (F,) {
    let t48469 = 0.85129199786595678799e-5 * t45394 + 0.23942587439980034662e-4 * t45396 - 0.32326021979378162576e-5 * t39370 - 0.212822999466489197e-4 * t45403 + 0.638468998399467591e-4 * t45407 - 0.638468998399467591e-4 * t45411 - 0.212822999466489197e-4 * t45415 + 0.59620292925746722033e-2 * t39388 - 0.8182398496273602519e-1 * t45420 - 0.425645998932978394e-4 * t45424 - 0.3405167991463827152e-4 * t45428 + 0.10215503974391481456e-3 * t45432 - 0.3405167991463827152e-4 * t45436 + 0.3405167991463827152e-4 * t45439 - 0.638468998399467591e-4 * t45441 + 0.5107751987195740728e-4 * t45446 + 0.5107751987195740728e-4 * t45451;
    (t48469,)
}
