//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 924/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk924<F: Float>(t11466: F, t11469: F, t11471: F, t11475: F, t11477: F, t11481: F, t11486: F, t11490: F, t11493: F, t11497: F, t11501: F, t11504: F, t11506: F, t11510: F, t11524: F, t11527: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12107 = 0.31675337336021900771e-5 * t11466;
    let t12108 = 0.67530371184977617164e-6 * t11469;
    let t12109 = 0.67530371184977617164e-6 * t11471;
    let t12110 = 0.40022999988963401107e-7 * t11475;
    let t12111 = 0.40096157891080460192e-6 * t11477;
    let t12112 = 0.16908181191593721013e-5 * t11481;
    let t12113 = 0.2318836277704281739e-4 * t11486;
    let t12114 = 0.4637672555408563478e-4 * t11490;
    let t12115 = 0.4637672555408563478e-4 * t11493;
    let t12116 = 0.38647271295071362317e-6 * t11497;
    let t12117 = 0.68714848362636882201e-6 * t11501;
    let t12118 = 0.22510123728325872388e-7 * t11504;
    let t12119 = 0.22510123728325872388e-6 * t11506;
    let t12120 = 0.30353495895471971565e-6 * t11510;
    let t12123 = 0.25301920572916666668e-5 * t11524;
    let t12124 = 0.25301920572916666668e-5 * t11527;
    (t12107, t12108, t12109, t12110, t12111, t12112, t12113, t12114, t12115, t12116, t12117, t12118, t12119, t12120, t12123, t12124)
}
