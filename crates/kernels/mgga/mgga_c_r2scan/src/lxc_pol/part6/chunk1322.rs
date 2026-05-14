//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1322/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1322<F: Float>(t24966: F, t1592: F, t2612: F, t551: F, t6343: F, t1584: F, t7597: F, t20151: F, t2223: F, t2252: F, t24943: F, t24945: F, t24948: F, t24955: F, t24963: F, t2526: F, t2646: F, t5054: F, t506: F, t529: F, t552: F, t6182: F, t6425: F, t6566: F, t7561: F, t8237: F, t910: F, t938: F) -> (F,) {
    let t24967 = 0.25426783770825854452e1 * t24966;
    let t24970 = t1592 * t551 * t6343 * t2612;
    let t24971 = 0.38140175656238781678e1 * t24970;
    let t24972 = t1584 * t7597;
    let t24973 = 0.12713391885412927226e1 * t24972;
    let t24990 = -0.10401866088065122276e1 * t24943 - 0.23049072140274290595e1 * t24945 - 0.48787202696913915093e-3 * t24948 + 0.2600466522016280569e1 * t20151 * t551 * t552 * t938 * t5054 + 0.49390868872016336991e0 * t2223 * t529 * t506 * t24955 - t24963 + t24967 + t24971 - t24973 - 0.13002332610081402845e0 * t6182 * t2646 - 0.13002332610081402845e0 * t1584 * t7561 + 0.39006997830244208535e0 * t6425 * t8237 + 0.39006997830244208535e0 * t1592 * t551 * t552 * t2526 * t2252 + 0.13002332610081402845e0 * t1592 * t551 * t552 * t910 * t6566;
    (t24990,)
}
