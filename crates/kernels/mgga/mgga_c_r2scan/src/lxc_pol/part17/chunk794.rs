//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 794/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk794<F: Float>(t3216: F, t560: F, t551: F, t552: F, t481: F, t3016: F, t2892: F, t506: F, t529: F, t2526: F, t910: F, t1584: F, t1592: F, t2184: F, t2196: F, t2223: F, t2640: F, t2646: F, t2651: F, t2656: F, t3068: F, t3077: F, t6073: F, t6075: F, t6084: F, t6097: F, t6105: F, t6205: F, t6522: F, t7383: F, t8240: F) -> (F, F, F, F) {
    let t9098 = t3216 * t560;
    let t9100 = t551 * t552 * t9098;
    let t9103 = t3216 * t481;
    let t9105 = t551 * t552 * t9103;
    let t9110 = t3016 * t560;
    let t9112 = t551 * t552 * t9110;
    let t9115 = t3016 * t481;
    let t9117 = t551 * t552 * t9115;
    let t9124 = t2892 * t481;
    let t9126 = t529 * t506 * t9124;
    let t9129 = t910 * t2526;
    let t9131 = t529 * t506 * t9129;
    let t9134 = 0.16262400898971305031e-3 * t6073 + 0.63479958930231934629e-2 * t6075 - t6084 + 0.42377972951376424087e0 * t6097 - t6105 + 0.2600466522016280569e0 * t8240 * t2656 + 0.10401866088065122276e1 * t7383 * t2640 + 0.86682217400542685632e-1 * t2184 * t9100 + 0.13002332610081402845e0 * t1592 * t9105 - 0.43341108700271342816e-1 * t1584 * t3068 + 0.13002332610081402845e0 * t1592 * t9112 + 0.5200933044032561138e0 * t2196 * t9117 + 0.86682217400542685632e-1 * t6205 * t3077 - 0.86682217400542685632e-1 * t2651 * t2646 - 0.65854491829355115988e0 * t6522 * t9126 + 0.32927245914677557994e0 * t2223 * t9131;
    (t9115, t9124, t9129, t9134)
}
