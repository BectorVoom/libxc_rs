//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 904/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk904<F: Float>(t551: F, t552: F, t6428: F, t1577: F, t1592: F, t1595: F, t535: F, t574: F, t6372: F, t6377: F, t6381: F, t6386: F, t6389: F, t6392: F, t6396: F, t6400: F, t6404: F, t6408: F, t6410: F, t6415: F, t6418: F, t6420: F, t6424: F, t6425: F) -> (F, F) {
    let t6430 = t551 * t552 * t6428;
    let t6433 = 0.26004665220162805689e0 * t1577 * t6372 - 0.16463622957338778997e0 * t535 * t6377 + 0.16463622957338778996e0 * t535 * t6381 + 0.34672886960217074253e0 * t6386 - 0.43341108700271342816e-1 * t574 * t6389 + 0.29272321618148349056e-1 * t6392 - 0.34930954652346593433e-1 * t6396 + 0.2037639021386884617e0 * t6400 - 0.17465477326173296717e-1 * t6404 + 0.87816964854445047168e-1 * t6408 - 0.14636160809074174528e-1 * t6410 + t6415 + 0.17465477326173296717e-1 * t6418 - 0.98781737744032673979e-1 * t6420 + t6424 + 0.7801399566048841707e0 * t6425 * t1595 + 0.39006997830244208535e0 * t1592 * t6430;
    (t6430, t6433)
}
