//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1302/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1302<F: Float>(t1551: F, t20881: F, t2122: F, t2124: F, t24527: F, t24543: F, t24547: F, t24549: F, t24555: F, t24558: F, t24565: F, t24568: F, t2550: F, t2557: F, t2572: F, t2582: F, t259: F, t2590: F, t2600: F, t360: F, t5074: F, t571: F, t6288: F, t6370: F, t7984: F, t7994: F) -> (F,) {
    let t24570 = 0.11524536070137145298e1 * t24527 + 0.16463622957338778996e0 * t2122 * t2124 * t7994 * t1551 + 0.13002332610081402845e0 * t7984 * t6288 + 0.26004665220162805689e0 * t571 * t20881 * t259 * t2600 - 0.27439371595564631661e-1 * t2557 * t2124 * t2550 * t5074 - 0.38415120233790484326e0 * t24543 + t24547 + 0.34672886960217074253e0 * t24549 - 0.43341108700271342816e-1 * t2582 * t360 * t2572 * t5074 + 0.19207560116895242163e0 * t24555 + 0.69345773920434148506e0 * t24558 + 0.38415120233790484324e0 * t2557 * t2124 * t2590 * t6370 - 0.20803732176130244552e1 * t24565 + 0.41607464352260489103e1 * t24568;
    (t24570,)
}
