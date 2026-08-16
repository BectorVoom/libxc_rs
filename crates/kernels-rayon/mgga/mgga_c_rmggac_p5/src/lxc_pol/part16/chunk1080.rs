//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1080/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1080(t39370: f64, t39388: f64, t45394: f64, t45396: f64, t45403: f64, t45407: f64, t45411: f64, t45415: f64, t45420: f64, t45424: f64, t45428: f64, t45432: f64, t45436: f64, t45439: f64, t45441: f64, t45446: f64, t45451: f64) -> f64 {
    let t48469 = 0.85129199786595678799e-5_f64 * t45394 + 0.23942587439980034662e-4_f64 * t45396 - 0.32326021979378162576e-5_f64 * t39370 - 0.212822999466489197e-4_f64 * t45403 + 0.638468998399467591e-4_f64 * t45407 - 0.638468998399467591e-4_f64 * t45411 - 0.212822999466489197e-4_f64 * t45415 + 0.59620292925746722033e-2_f64 * t39388 - 0.8182398496273602519e-1_f64 * t45420 - 0.425645998932978394e-4_f64 * t45424 - 0.3405167991463827152e-4_f64 * t45428 + 0.10215503974391481456e-3_f64 * t45432 - 0.3405167991463827152e-4_f64 * t45436 + 0.3405167991463827152e-4_f64 * t45439 - 0.638468998399467591e-4_f64 * t45441 + 0.5107751987195740728e-4_f64 * t45446 + 0.5107751987195740728e-4_f64 * t45451;
    t48469
}
