//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 691/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk691<F: Float>(t1457: F, t6443: F, t2335: F, t4673: F, t2398: F, t4614: F, t2378: F, t6429: F, t1265: F, t2334: F, t1445: F, t1441: F, t1450: F, t1456: F, t1520: F, t1537: F, t1572: F, t1580: F, t193: F, t2402: F, t2434: F, t4631: F, t4762: F, t4781: F, t4811: F, t567: F, t574: F, t6756: F, t6760: F, t6764: F, t6768: F, t6773: F, t6777: F, t6785: F, t6790: F, t6793: F, t895: F, t904: F) -> (F, F, F) {
    let t6795 = t1457 * t6443;
    let t6798 = t4673 * t2335;
    let t6801 = t4614 * t2398;
    let t6804 = t4614 * t2378;
    let t6807 = t1457 * t6429;
    let t6810 = t2334 * t1265;
    let t6811 = t1445 * t6810;
    let t6814 = 0.1022478025437886658e1 * t4811 * t6756 + 0.30674340763136599742e1 * t4781 * t6760 + 0.2044956050875773316e1 * t1441 * t6764 - 0.51123901271894332902e1 * t1537 * t6768 - 0.79445533226334281487e-1 * t895 * t1520 + 0.71500979903700853338e0 * t6773 * t193 + 0.35750489951850426669e0 * t6777 * t193 - 0.35750489951850426669e0 * t4631 * t904 + 0.46011511144704899612e1 * t1580 * t2434 - 0.23005755572352449806e1 * t1450 * t6785 - 0.35750489951850426669e0 * t2402 * t4762 + 0.23005755572352449806e1 * t567 * t6790 - 0.59584149919750711116e-1 * t6793 + 0.71500979903700853338e0 * t1572 * t6795 + 0.47667319935800568892e0 * t1456 * t6798 - 0.61348681526273199482e1 * t1450 * t6801 - 0.12269736305254639896e2 * t574 * t6804 + 0.14300195980740170668e1 * t1572 * t6807 - 0.46011511144704899612e1 * t574 * t6811;
    (t6795, t6798, t6814)
}
