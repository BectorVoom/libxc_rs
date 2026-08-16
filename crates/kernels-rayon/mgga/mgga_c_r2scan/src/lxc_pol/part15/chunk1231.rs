//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1231/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1231(t2337: f64, t2881: f64, t3424: f64, t3685: f64, t40443: f64, t40444: f64, t40446: f64, t40448: f64, t40463: f64, t40467: f64, t40469: f64, t40471: f64, t40475: f64, t40479: f64, t40483: f64, t40490: f64, t40495: f64, t40502: f64) -> f64 {
    let t40735 = t2337 * t3685 + 2.0_f64 * t2881 * t3424 + t40443 + t40444 - t40446 - t40448 - t40463 - t40467 + t40469 + t40471 - t40475 - t40479 + t40483 - t40490 - t40495 + t40502;
    t40735
}
