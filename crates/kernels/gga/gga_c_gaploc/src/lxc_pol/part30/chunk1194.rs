//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1194/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1194<F: Float>(t2437: F, t8063: F, t14626: F, t3390: F, t574: F, t10382: F, t1580: F, t10123: F, t10340: F, t1265: F, t1445: F, t1456: F, t1457: F, t1562: F, t1603: F, t31509: F, t31870: F, t3371: F, t34070: F, t34074: F, t34078: F, t34087: F, t34092: F, t34094: F, t34096: F, t34098: F, t4667: F, t4673: F, t4842: F) -> (F,) {
    let t34100 = 0.47667319935800568892e0 * t2437 * t8063;
    let t34106 = 0.20449560508757733161e1 * t574 * t14626 * t3390;
    let t34108 = 0.23005755572352449806e2 * t1580 * t10382;
    let t34112 = -t34070 - t34074 - t34078 - 0.69017266717057349418e1 * t1562 * t1445 * t10340 * t1265 - 0.71500979903700853338e0 * t4842 * t1457 * t31870 - t34087 + 0.47667319935800568892e0 * t1456 * t4673 * t10123 + t34092 + t34094 + t34096 + t34098 + t34100 + 0.35750489951850426669e0 * t1456 * t1457 * t31509 - t34106 + t34108 + 0.71500979903700853338e0 * t1603 * t3371 * t4667;
    (t34112,)
}
