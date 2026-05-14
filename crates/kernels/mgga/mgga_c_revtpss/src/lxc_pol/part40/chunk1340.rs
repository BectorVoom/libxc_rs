//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1340/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1340<F: Float>(t31027: F, t31440: F, t31032: F, t31444: F, t108: F, t1513: F, t116912: F, t31417: F, t31421: F, t101460: F, t10199: F, t117183: F, t117184: F, t117186: F, t117188: F, t117190: F, t117198: F, t117218: F, t117226: F, t117544: F, t117545: F, t1509: F, t2194: F, t2358: F, t2362: F, t2366: F, t31035: F, t31142: F, t31149: F, t31433: F, t36308: F, t36315: F, t4279: F, t8258: F, t8267: F, t8311: F, t8315: F) -> (F,) {
    let t117976 = 20.0 / 9.0 * t31027 * t31440;
    let t117978 = 20.0 / 27.0 * t31032 * t31444;
    let t117997 = t108 * t1513;
    let t118009 = 4.0 * t116912 * t31417;
    let t118011 = 20.0 / 9.0 * t31027 * t31421;
    let t118017 = -25.0 / 18.0 * t8258 * t31433 * t31142 - t117976 + t117978 + 5.0 / 12.0 * t8258 * t8315 * t1509 * t2366 - 5.0 / 6.0 * t117544 * t8315 * t117545 - 5.0 / 36.0 * t8267 * t31149 * t1509 * t2362 + 5.0 / 24.0 * t10199 * t2194 * t108 + 44.0 / 9.0 * t117184 - 110.0 / 27.0 * t117186 - 2.0 / 3.0 * t117188 + 5.0 / 9.0 * t117190 + t117183 - 5.0 / 2.0 * t36308 * t117997 * t31142 + 5.0 / 9.0 * t36315 * t4279 * t31142 + 5.0 / 108.0 * t8267 * t117218 * t1509 * t2358 + t118009 - t118011 - 3.0 / 2.0 * t31035 * t8311 * t101460 + 10.0 / 27.0 * t117198 + 2.0 * t117226;
    (t118017,)
}
