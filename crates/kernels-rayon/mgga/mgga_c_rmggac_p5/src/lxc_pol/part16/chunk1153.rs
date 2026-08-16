//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1153/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1153(t10338: f64, t10339: f64, t10340: f64, t10341: f64, t10342: f64, t10343: f64, t10344: f64, t10345: f64, t37148: f64, t42492: f64, t42493: f64, t42501: f64, t42502: f64, t42504: f64, t42505: f64, t42506: f64, t42507: f64, t42508: f64, t8692: f64, t8694: f64, t8696: f64, t8698: f64) -> (f64, f64) {
    let t49853 = t37148 - t42492 - t10338 - t10339 + t10340 + t10341 + t10342 - t10343 - t42493 - t10344 - t10345;
    let t49862 = -t42501 - t42502 + t42504 - t42505 - t42506 + t42507 + 0.79453919800822633545e-4_f64 * t8692 + 0.23836175940246790064e-3_f64 * t8694 - 0.23836175940246790064e-3_f64 * t8696 + 0.79453919800822633545e-4_f64 * t8698 + t42508;
    (t49853, t49862)
}
