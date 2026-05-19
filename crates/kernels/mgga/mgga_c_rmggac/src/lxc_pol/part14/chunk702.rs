//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 702/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk702<F: Float>(t8520: F, t8543: F, t8546: F, t8549: F, t8552: F, t8612: F, t8617: F, t8655: F, t8669: F, t8677: F, t8692: F, t8698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9309 = F::cast_from(0.23942587439980034662e-4_f64) * t8520;
    let t9335 = F::cast_from(0.5987120850931904282e-1_f64) * t8543;
    let t9336 = F::cast_from(0.17961362552795712846e0_f64) * t8546;
    let t9337 = F::cast_from(0.35922725105591425692e0_f64) * t8549;
    let t9338 = F::cast_from(0.8980681276397856423e-1_f64) * t8552;
    let t9368 = F::cast_from(0.1064114997332445985e-4_f64) * t8612;
    let t9369 = F::cast_from(0.1064114997332445985e-4_f64) * t8617;
    let t9381 = F::cast_from(0.2993560425465952141e-1_f64) * t8655;
    let t9393 = F::cast_from(0.1064114997332445985e-4_f64) * t8669;
    let t9412 = F::cast_from(0.1064114997332445985e-4_f64) * t8677;
    let t9419 = F::cast_from(0.19863479950205658386e-4_f64) * t8692;
    let t9422 = F::cast_from(0.19863479950205658386e-4_f64) * t8698;
    (t9309, t9335, t9336, t9337, t9338, t9368, t9369, t9381, t9393, t9412, t9419, t9422)
}
