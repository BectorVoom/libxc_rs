//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1118/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1118<F: Float>(t1737: F, t2228: F, t1364: F, t1627: F, t2471: F, t41727: F, t41735: F, t41767: F, t43810: F, t43812: F, t43813: F, t43836: F, t47435: F, t47439: F, t47442: F, t47445: F, t47450: F, t47455: F, t47460: F, t739: F, t8377: F, t903: F, t9530: F) -> (F, F) {
    let t49184 = t2228 * t1737;
    let t49199 = -F::cast_from(0.72042316457491791901e-3_f64) * t47435 - F::cast_from(0.60975299583150056624e-3_f64) * t47439 - F::cast_from(0.1440846329149835838e-2_f64) * t47442 + t43810 - t43812 - t43813 + F::cast_from(0.1333427903096438929e0_f64) * t41727 - F::cast_from(0.23948483403727617128e0_f64) * t1364 * t49184 - F::cast_from(0.72732431077987577948e-1_f64) * t41735 + F::cast_from(0.2993560425465952141e-1_f64) * t47445 - F::cast_from(0.10215503974391481456e-3_f64) * t47450 - t43836 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2471 * t1627 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t9530 * t8377 + F::cast_from(0.13242319966803772257e-3_f64) * t41767 + F::cast_from(0.1064114997332445985e-4_f64) * t47455 + F::cast_from(0.212822999466489197e-4_f64) * t47460;
    (t49184, t49199)
}
