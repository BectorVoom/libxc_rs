//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 895/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk895(t506: f64, t529: f64, t7569: f64, t2252: f64, t938: f64, t551: f64, t552: f64, t1234: f64, t2627: f64, t6518: f64, t1579: f64, t1592: f64, t2184: f64, t2223: f64, t535: f64, t6182: f64, t6410: f64, t6415: f64, t6418: f64, t6420: f64, t6424: f64, t6440: f64, t6446: f64, t6522: f64, t8189: f64, t8191: f64, t8198: f64, t8201: f64, t8204: f64, t948: f64) -> f64 {
    let t8210 = t529 * t506 * t7569;
    let t8213 = t938 * t2252;
    let t8215 = t551 * t552 * t8213;
    let t8218 = t938 * t1234;
    let t8220 = t551 * t552 * t8218;
    let t8224 = 0.76830240467580968652e0_f64 * t6518 * t2627;
    let t8225 = -0.48787202696913915093e-2_f64 * t6410 + t6415 + 0.58218257753910989057e-2_f64 * t6418 - 0.65854491829355115987e-1_f64 * t6420 + t6424 + t8189 - 0.65854491829355115988e0_f64 * t6522 * t8191 + 0.23115257973478049502e0_f64 * t6440 + 0.11557628986739024751e0_f64 * t6446 + 0.86682217400542685632e-1_f64 * t8198 * t1579 + 0.42377972951376424087e0_f64 * t8201 - 0.27439371595564631661e-1_f64 * t535 * t8204 - 0.43341108700271342816e-1_f64 * t6182 * t948 + 0.16463622957338778997e0_f64 * t2223 * t8210 + 0.86682217400542685632e-1_f64 * t2184 * t8215 + 0.13002332610081402845e0_f64 * t1592 * t8220 - t8224;
    t8225
}
