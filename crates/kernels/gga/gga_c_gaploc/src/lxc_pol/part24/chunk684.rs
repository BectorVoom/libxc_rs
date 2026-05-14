//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 684/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk684<F: Float>(t1445: F, t6672: F, t1603: F, t894: F, t2345: F, t4614: F, t2417: F, t1457: F, t6424: F, t1456: F, t1562: F, t1580: F, t1641: F, t2379: F, t2382: F, t2386: F, t2392: F, t2402: F, t2418: F, t4667: F, t4730: F, t4753: F, t4950: F, t4953: F, t567: F, t574: F, t597: F, t6637: F, t6642: F, t6649: F, t6652: F, t6656: F, t6659: F, t6665: F, t6669: F) -> (F, F) {
    let t6673 = t1445 * t6672;
    let t6676 = t1603 * t894;
    let t6679 = t4614 * t2345;
    let t6682 = t4614 * t2417;
    let t6689 = t1457 * t6424;
    let t6692 = 0.61348681526273199482e1 * t567 * t6637 - 0.47667319935800568892e0 * t2402 * t4753 - 0.21450293971110256002e1 * t6642 * t2386 + 0.14300195980740170668e1 * t4950 * t2392 - 0.13803453343411469884e2 * t1562 * t6649 - 0.21450293971110256002e1 * t6652 * t2386 - 0.92023022289409799224e1 * t574 * t6656 + 0.23005755572352449806e2 * t597 * t6659 - 0.13803453343411469884e2 * t4953 * t2418 - 0.92023022289409799224e1 * t574 * t6665 + 0.43710935587469654631e2 * t597 * t6669 + 0.46011511144704899612e1 * t4730 * t6673 + 0.71500979903700853338e0 * t6676 * t4667 + 0.30674340763136599741e2 * t597 * t6679 - 0.18404604457881959845e2 * t1562 * t6682 - 0.92023022289409799224e1 * t1641 * t2379 + 0.23005755572352449806e2 * t1580 * t2382 + 0.35750489951850426669e0 * t1456 * t6689;
    (t6689, t6692)
}
