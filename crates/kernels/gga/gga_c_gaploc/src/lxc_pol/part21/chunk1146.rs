//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1146/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1146<F: Float>(t2877: F, t6777: F, t2441: F, t8072: F, t8063: F, t26279: F, t895: F, t2437: F, t14626: F, t3390: F, t574: F, t10382: F, t1580: F, t14630: F, t2859: F, t888: F) -> (F, F, F, F, F, F, F, F) {
    let t34092 = 0.35750489951850426669e0 * t6777 * t2877;
    let t34094 = 0.71500979903700853338e0 * t2441 * t8072;
    let t34096 = 0.47667319935800568892e0 * t2441 * t8063;
    let t34098 = 0.47667319935800568892e0 * t895 * t26279;
    let t34100 = 0.47667319935800568892e0 * t2437 * t8063;
    let t34106 = 0.20449560508757733161e1 * t574 * t14626 * t3390;
    let t34108 = 0.23005755572352449806e2 * t1580 * t10382;
    let t34119 = 0.23833659967900284447e0 * t2859 * t14630 * t888;
    (t34092, t34094, t34096, t34098, t34100, t34106, t34108, t34119)
}
