//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1153/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1153<F: Float>(t10338: F, t10339: F, t10340: F, t10341: F, t10342: F, t10343: F, t10344: F, t10345: F, t37148: F, t42492: F, t42493: F, t42501: F, t42502: F, t42504: F, t42505: F, t42506: F, t42507: F, t42508: F, t8692: F, t8694: F, t8696: F, t8698: F) -> (F, F) {
    let t49853 = t37148 - t42492 - t10338 - t10339 + t10340 + t10341 + t10342 - t10343 - t42493 - t10344 - t10345;
    let t49862 = -t42501 - t42502 + t42504 - t42505 - t42506 + t42507 + F::cast_from(0.79453919800822633545e-4_f64) * t8692 + F::cast_from(0.23836175940246790064e-3_f64) * t8694 - F::cast_from(0.23836175940246790064e-3_f64) * t8696 + F::cast_from(0.79453919800822633545e-4_f64) * t8698 + t42508;
    (t49853, t49862)
}
