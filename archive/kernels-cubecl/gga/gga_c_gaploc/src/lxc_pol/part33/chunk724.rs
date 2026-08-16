//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 724/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk724<F: Float>(t1457: F, t6429: F, t1265: F, t2334: F, t1445: F, t1441: F, t1450: F, t1456: F, t1520: F, t1537: F, t1572: F, t1580: F, t193: F, t2402: F, t2434: F, t4631: F, t4762: F, t4781: F, t4811: F, t567: F, t574: F, t6756: F, t6760: F, t6764: F, t6768: F, t6773: F, t6777: F, t6785: F, t6790: F, t6793: F, t6795: F, t6798: F, t6801: F, t6804: F, t895: F, t904: F) -> F {
    let t6807 = t1457 * t6429;
    let t6810 = t2334 * t1265;
    let t6811 = t1445 * t6810;
    let t6814 = F::cast_from(0.1022478025437886658e1_f64) * t4811 * t6756 + F::cast_from(0.30674340763136599742e1_f64) * t4781 * t6760 + F::cast_from(0.2044956050875773316e1_f64) * t1441 * t6764 - F::cast_from(0.51123901271894332902e1_f64) * t1537 * t6768 - F::cast_from(0.79445533226334281487e-1_f64) * t895 * t1520 + F::cast_from(0.71500979903700853338e0_f64) * t6773 * t193 + F::cast_from(0.35750489951850426669e0_f64) * t6777 * t193 - F::cast_from(0.35750489951850426669e0_f64) * t4631 * t904 + F::cast_from(0.46011511144704899612e1_f64) * t1580 * t2434 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t6785 - F::cast_from(0.35750489951850426669e0_f64) * t2402 * t4762 + F::cast_from(0.23005755572352449806e1_f64) * t567 * t6790 - F::cast_from(0.59584149919750711116e-1_f64) * t6793 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t6795 + F::cast_from(0.47667319935800568892e0_f64) * t1456 * t6798 - F::cast_from(0.61348681526273199482e1_f64) * t1450 * t6801 - F::cast_from(0.12269736305254639896e2_f64) * t574 * t6804 + F::cast_from(0.14300195980740170668e1_f64) * t1572 * t6807 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t6811;
    t6814
}
