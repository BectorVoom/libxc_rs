//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 827/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk827(t625: f64, t923: f64, t6462: f64, t113: f64, t2252: f64, t2572: f64, t360: f64, t2530: f64, t277: f64) -> (f64, f64, f64, f64, f64) {
    let t7418 = t923 * t625;
    let t7419 = t6462 * t7418;
    let t7428 = t113 * t2252;
    let t7429 = t2572 * t7428;
    let t7430 = t360 * t7429;
    let t7433 = t277 * t2530;
    (t7418, t7419, t7429, t7430, t7433)
}
