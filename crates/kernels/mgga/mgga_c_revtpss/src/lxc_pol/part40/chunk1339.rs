//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1339/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1339<F: Float>(t31027: F, t31430: F, t31032: F, t31434: F, t117461: F, t31447: F, t2357: F, t55: F, t116929: F, t8402: F, t116926: F, t8395: F, t2289: F, t8399: F, t31424: F, t101457: F, t101463: F, t116919: F, t117228: F, t13509: F, t1509: F, t1513: F, t2: F, t2340: F, t2358: F, t2362: F, t2366: F, t31035: F, t31149: F, t31287: F, t31429: F, t31433: F, t4287: F, t661: F, t8258: F, t8267: F, t8311: F, t8315: F) -> (F,) {
    let t117918 = 20.0 / 9.0 * t31027 * t31430;
    let t117920 = 50.0 / 27.0 * t31032 * t31434;
    let t117927 = t117461 * t31447;
    let t117932 = t55 * t2357;
    let t117936 = t116929 * t8402;
    let t117938 = t116926 * t8395;
    let t117940 = t2289 * t8399;
    let t117943 = 4.0 / 3.0 * t31027 * t31424;
    let t117971 = t8258 * t8311 * t13509 / 4.0 + t117918 - t117920 - 5.0 / 12.0 * t8258 * t31429 * t2366 + 25.0 / 72.0 * t8267 * t31433 * t2362 - 125.0 / 72.0 * t117927 + 5.0 / 4.0 * t31035 * t31429 * t2340 + 25.0 / 108.0 * t8267 * t117932 * t2358 - 55.0 / 27.0 * t117936 + 22.0 / 9.0 * t117938 + 55.0 / 27.0 * t117940 - t117943 - 20.0 / 9.0 * t117228 + 3.0 * t116919 * t8311 * t101457 + 5.0 / 18.0 * t8258 * t31149 * t1513 * t2358 - 5.0 / 4.0 * t31035 * t8315 * t1509 * t2340 + 5.0 / 18.0 * t31287 * t31149 * t2 * t661 - 3.0 / 4.0 * t31035 * t8311 * t101463 + 5.0 / 6.0 * t8258 * t8315 * t4287 * t661 + 5.0 / 12.0 * t8258 * t8315 * t1513 * t2362;
    (t117971,)
}
