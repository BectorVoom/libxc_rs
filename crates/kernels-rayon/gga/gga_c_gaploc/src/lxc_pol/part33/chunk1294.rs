//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1294/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1294(t2877: f64, t6777: f64, t2441: f64, t8072: f64, t8063: f64, t26279: f64, t895: f64, t2437: f64, t14626: f64, t3390: f64, t574: f64, t10382: f64, t1580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34092 = 0.35750489951850426669e0_f64 * t6777 * t2877;
    let t34094 = 0.71500979903700853338e0_f64 * t2441 * t8072;
    let t34096 = 0.47667319935800568892e0_f64 * t2441 * t8063;
    let t34098 = 0.47667319935800568892e0_f64 * t895 * t26279;
    let t34100 = 0.47667319935800568892e0_f64 * t2437 * t8063;
    let t34106 = 0.20449560508757733161e1_f64 * t574 * t14626 * t3390;
    let t34108 = 0.23005755572352449806e2_f64 * t1580 * t10382;
    (t34092, t34094, t34096, t34098, t34100, t34106, t34108)
}
