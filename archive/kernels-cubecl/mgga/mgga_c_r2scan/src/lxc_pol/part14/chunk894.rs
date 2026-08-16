//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 894/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk894<F: Float>(t506: F, t529: F, t7569: F, t2252: F, t938: F, t551: F, t552: F, t1234: F, t2627: F, t6518: F, t1579: F, t1592: F, t2184: F, t2223: F, t535: F, t6182: F, t6410: F, t6415: F, t6418: F, t6420: F, t6424: F, t6440: F, t6446: F, t6522: F, t8189: F, t8191: F, t8198: F, t8201: F, t8204: F, t948: F) -> F {
    let t8210 = t529 * t506 * t7569;
    let t8213 = t938 * t2252;
    let t8215 = t551 * t552 * t8213;
    let t8218 = t938 * t1234;
    let t8220 = t551 * t552 * t8218;
    let t8224 = F::cast_from(0.76830240467580968652e0_f64) * t6518 * t2627;
    let t8225 = -F::cast_from(0.48787202696913915093e-2_f64) * t6410 + t6415 + F::cast_from(0.58218257753910989057e-2_f64) * t6418 - F::cast_from(0.65854491829355115987e-1_f64) * t6420 + t6424 + t8189 - F::cast_from(0.65854491829355115988e0_f64) * t6522 * t8191 + F::cast_from(0.23115257973478049502e0_f64) * t6440 + F::cast_from(0.11557628986739024751e0_f64) * t6446 + F::cast_from(0.86682217400542685632e-1_f64) * t8198 * t1579 + F::cast_from(0.42377972951376424087e0_f64) * t8201 - F::cast_from(0.27439371595564631661e-1_f64) * t535 * t8204 - F::cast_from(0.43341108700271342816e-1_f64) * t6182 * t948 + F::cast_from(0.16463622957338778997e0_f64) * t2223 * t8210 + F::cast_from(0.86682217400542685632e-1_f64) * t2184 * t8215 + F::cast_from(0.13002332610081402845e0_f64) * t1592 * t8220 - t8224;
    t8225
}
