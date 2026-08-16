//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 862/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk862(t3216: f64, t560: f64, t551: f64, t552: f64, t481: f64, t3016: f64, t2892: f64, t506: f64, t529: f64, t2526: f64, t910: f64, t1584: f64, t1592: f64, t2184: f64, t2196: f64, t2223: f64, t2640: f64, t2646: f64, t2651: f64, t2656: f64, t3068: f64, t3077: f64, t6073: f64, t6075: f64, t6084: f64, t6097: f64, t6105: f64, t6205: f64, t6522: f64, t7383: f64, t8240: f64) -> (f64, f64, f64, f64) {
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
    let t9134 = 0.16262400898971305031e-3_f64 * t6073 + 0.63479958930231934629e-2_f64 * t6075 - t6084 + 0.42377972951376424087e0_f64 * t6097 - t6105 + 0.2600466522016280569e0_f64 * t8240 * t2656 + 0.10401866088065122276e1_f64 * t7383 * t2640 + 0.86682217400542685632e-1_f64 * t2184 * t9100 + 0.13002332610081402845e0_f64 * t1592 * t9105 - 0.43341108700271342816e-1_f64 * t1584 * t3068 + 0.13002332610081402845e0_f64 * t1592 * t9112 + 0.5200933044032561138e0_f64 * t2196 * t9117 + 0.86682217400542685632e-1_f64 * t6205 * t3077 - 0.86682217400542685632e-1_f64 * t2651 * t2646 - 0.65854491829355115988e0_f64 * t6522 * t9126 + 0.32927245914677557994e0_f64 * t2223 * t9131;
    (t9115, t9124, t9129, t9134)
}
