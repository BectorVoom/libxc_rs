//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1333/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1333<F: Float>(t255: F, t537: F, t571: F, t7916: F, t4933: F, t910: F, t1632: F, t2184: F, t551: F, t8213: F, t1588: F, t1592: F, t2196: F, t24156: F, t24955: F, t24996: F, t25001: F, t25088: F, t2651: F, t552: F, t560: F, t574: F, t576: F, t6225: F, t6493: F, t6572: F, t6576: F, t7088: F, t7566: F, t7571: F, t7984: F, t7987: F) -> (F, F) {
    let t25095 = t571 * t537 * t7916 * t255;
    let t25107 = t910 * t4933;
    let t25114 = t2184 * t551 * t1632 * t8213;
    let t25128 = 0.15602799132097683414e1 * t6493 * t7571 - t24996 - t25001 - 0.43341108700271342816e-1 * t574 * t551 * t552 * t25088 - 0.13002332610081402845e0 * t25095 * t576 + 0.39006997830244208535e0 * t1592 * t551 * t552 * t7088 * t560 + 0.15602799132097683414e1 * t2196 * t551 * t552 * t24955 + 0.5200933044032561138e0 * t2196 * t551 * t552 * t25107 - 0.69345773920434148506e0 * t25114 + 0.15602799132097683414e1 * t2196 * t551 * t552 * t24156 - 0.13002332610081402845e0 * t7566 * t1588 - 0.43341108700271342816e-1 * t2651 * t6225 + 0.13002332610081402845e0 * t7984 * t6572 + 0.39006997830244208535e0 * t7987 * t6576;
    (t25107, t25128)
}
