//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1051/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1051<F: Float>(t11466: F, t11469: F, t11471: F, t11475: F, t11477: F, t11481: F, t11486: F, t11490: F, t11493: F, t11497: F, t11501: F, t11504: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12107 = F::cast_from(0.31675337336021900771e-5_f64) * t11466;
    let t12108 = F::cast_from(0.67530371184977617164e-6_f64) * t11469;
    let t12109 = F::cast_from(0.67530371184977617164e-6_f64) * t11471;
    let t12110 = F::cast_from(0.40022999988963401107e-7_f64) * t11475;
    let t12111 = F::cast_from(0.40096157891080460192e-6_f64) * t11477;
    let t12112 = F::cast_from(0.16908181191593721013e-5_f64) * t11481;
    let t12113 = F::cast_from(0.2318836277704281739e-4_f64) * t11486;
    let t12114 = F::cast_from(0.4637672555408563478e-4_f64) * t11490;
    let t12115 = F::cast_from(0.4637672555408563478e-4_f64) * t11493;
    let t12116 = F::cast_from(0.38647271295071362317e-6_f64) * t11497;
    let t12117 = F::cast_from(0.68714848362636882201e-6_f64) * t11501;
    let t12118 = F::cast_from(0.22510123728325872388e-7_f64) * t11504;
    (t12107, t12108, t12109, t12110, t12111, t12112, t12113, t12114, t12115, t12116, t12117, t12118)
}
