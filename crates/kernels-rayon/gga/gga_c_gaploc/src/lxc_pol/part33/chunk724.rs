//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 724/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk724(t1457: f64, t6429: f64, t1265: f64, t2334: f64, t1445: f64, t1441: f64, t1450: f64, t1456: f64, t1520: f64, t1537: f64, t1572: f64, t1580: f64, t193: f64, t2402: f64, t2434: f64, t4631: f64, t4762: f64, t4781: f64, t4811: f64, t567: f64, t574: f64, t6756: f64, t6760: f64, t6764: f64, t6768: f64, t6773: f64, t6777: f64, t6785: f64, t6790: f64, t6793: f64, t6795: f64, t6798: f64, t6801: f64, t6804: f64, t895: f64, t904: f64) -> f64 {
    let t6807 = t1457 * t6429;
    let t6810 = t2334 * t1265;
    let t6811 = t1445 * t6810;
    let t6814 = 0.1022478025437886658e1_f64 * t4811 * t6756 + 0.30674340763136599742e1_f64 * t4781 * t6760 + 0.2044956050875773316e1_f64 * t1441 * t6764 - 0.51123901271894332902e1_f64 * t1537 * t6768 - 0.79445533226334281487e-1_f64 * t895 * t1520 + 0.71500979903700853338e0_f64 * t6773 * t193 + 0.35750489951850426669e0_f64 * t6777 * t193 - 0.35750489951850426669e0_f64 * t4631 * t904 + 0.46011511144704899612e1_f64 * t1580 * t2434 - 0.23005755572352449806e1_f64 * t1450 * t6785 - 0.35750489951850426669e0_f64 * t2402 * t4762 + 0.23005755572352449806e1_f64 * t567 * t6790 - 0.59584149919750711116e-1_f64 * t6793 + 0.71500979903700853338e0_f64 * t1572 * t6795 + 0.47667319935800568892e0_f64 * t1456 * t6798 - 0.61348681526273199482e1_f64 * t1450 * t6801 - 0.12269736305254639896e2_f64 * t574 * t6804 + 0.14300195980740170668e1_f64 * t1572 * t6807 - 0.46011511144704899612e1_f64 * t574 * t6811;
    t6814
}
