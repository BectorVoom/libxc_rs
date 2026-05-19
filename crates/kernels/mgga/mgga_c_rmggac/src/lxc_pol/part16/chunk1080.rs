//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1080/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1080<F: Float>(t39370: F, t39388: F, t45394: F, t45396: F, t45403: F, t45407: F, t45411: F, t45415: F, t45420: F, t45424: F, t45428: F, t45432: F, t45436: F, t45439: F, t45441: F, t45446: F, t45451: F) -> F {
    let t48469 = F::cast_from(0.85129199786595678799e-5_f64) * t45394 + F::cast_from(0.23942587439980034662e-4_f64) * t45396 - F::cast_from(0.32326021979378162576e-5_f64) * t39370 - F::cast_from(0.212822999466489197e-4_f64) * t45403 + F::cast_from(0.638468998399467591e-4_f64) * t45407 - F::cast_from(0.638468998399467591e-4_f64) * t45411 - F::cast_from(0.212822999466489197e-4_f64) * t45415 + F::cast_from(0.59620292925746722033e-2_f64) * t39388 - F::cast_from(0.8182398496273602519e-1_f64) * t45420 - F::cast_from(0.425645998932978394e-4_f64) * t45424 - F::cast_from(0.3405167991463827152e-4_f64) * t45428 + F::cast_from(0.10215503974391481456e-3_f64) * t45432 - F::cast_from(0.3405167991463827152e-4_f64) * t45436 + F::cast_from(0.3405167991463827152e-4_f64) * t45439 - F::cast_from(0.638468998399467591e-4_f64) * t45441 + F::cast_from(0.5107751987195740728e-4_f64) * t45446 + F::cast_from(0.5107751987195740728e-4_f64) * t45451;
    t48469
}
