//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1351/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1351(t2441: f64, t8072: f64, t8063: f64, t26279: f64, t895: f64, t2437: f64, t14626: f64, t3390: f64, t574: f64, t10382: f64, t1580: f64, t10123: f64, t10340: f64, t1265: f64, t1445: f64, t1456: f64, t1457: f64, t1562: f64, t1603: f64, t31509: f64, t31870: f64, t3371: f64, t34070: f64, t34074: f64, t34078: f64, t34087: f64, t34092: f64, t4667: f64, t4673: f64, t4842: f64) -> f64 {
    let t34094 = 0.71500979903700853338e0_f64 * t2441 * t8072;
    let t34096 = 0.47667319935800568892e0_f64 * t2441 * t8063;
    let t34098 = 0.47667319935800568892e0_f64 * t895 * t26279;
    let t34100 = 0.47667319935800568892e0_f64 * t2437 * t8063;
    let t34106 = 0.20449560508757733161e1_f64 * t574 * t14626 * t3390;
    let t34108 = 0.23005755572352449806e2_f64 * t1580 * t10382;
    let t34112 = -t34070 - t34074 - t34078 - 0.69017266717057349418e1_f64 * t1562 * t1445 * t10340 * t1265 - 0.71500979903700853338e0_f64 * t4842 * t1457 * t31870 - t34087 + 0.47667319935800568892e0_f64 * t1456 * t4673 * t10123 + t34092 + t34094 + t34096 + t34098 + t34100 + 0.35750489951850426669e0_f64 * t1456 * t1457 * t31509 - t34106 + t34108 + 0.71500979903700853338e0_f64 * t1603 * t3371 * t4667;
    t34112
}
