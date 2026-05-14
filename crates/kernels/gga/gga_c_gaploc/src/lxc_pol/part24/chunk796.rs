//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 796/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk796<F: Float>(t2779: F, t4614: F, t1323: F, t2787: F, t1445: F, t1603: F, t999: F, t1457: F, t7957: F, t493: F, t7892: F, t590: F, t1339: F, t4130: F, t7905: F, t1436: F, t1441: F, t1537: F, t1596: F, t2872: F, t2877: F, t4667: F, t4730: F, t4753: F, t4781: F, t4811: F, t4842: F, t567: F, t6580: F, t6585: F, t6587: F, t6591: F, t6594: F, t6597: F, t6601: F) -> (F, F) {
    let t8176 = t4614 * t2779;
    let t8179 = t2787 * t1323;
    let t8180 = t1445 * t8179;
    let t8183 = t1603 * t999;
    let t8190 = t1457 * t7957;
    let t8195 = t493 * t7892;
    let t8196 = t8195 * t590;
    let t8199 = t1339 * t7892;
    let t8200 = t8199 * t590;
    let t8204 = t4130 * t7892 * t590;
    let t8207 = t493 * t7905;
    let t8208 = t8207 * t590;
    let t8212 = t1339 * t7905 * t590;
    let t8220 = 0.61348681526273199482e1 * t567 * t8176 + 0.46011511144704899612e1 * t4730 * t8180 + 0.71500979903700853338e0 * t8183 * t4667 - 0.47667319935800568892e0 * t2872 * t4753 + 0.35750489951850426669e0 * t1596 * t2877 - 0.71500979903700853338e0 * t4842 * t8190 + 0.76685851907841499352e0 * t6580 - 0.76685851907841499352e0 * t6585 + 0.2044956050875773316e1 * t1441 * t8196 - 0.51123901271894332902e1 * t1537 * t8200 + 0.30674340763136599742e1 * t4781 * t8204 - 0.1022478025437886658e1 * t1436 * t8208 + 0.1022478025437886658e1 * t4811 * t8212 + 0.51123901271894332902e0 * t6587 - 0.89376224879626066674e-1 * t6591 + 0.11916829983950142223e0 * t6594 + 0.11916829983950142223e0 * t6597 - 0.29792074959875355558e-1 * t6601;
    (t8195, t8220)
}
