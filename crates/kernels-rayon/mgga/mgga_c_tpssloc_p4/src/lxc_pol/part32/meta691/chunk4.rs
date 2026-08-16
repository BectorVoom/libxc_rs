//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2141/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2141(t225: f64, t28051: f64, t1386: f64, t20044: f64, t2016: f64, t28187: f64, t3758: f64, t56640: f64, t6993: f64, t90525: f64, t90534: f64, t90542: f64, t90547: f64, t90550: f64, t96905: f64, t96910: f64) -> f64 {
    let t96913 = t28051 * t225;
    let t96917 = -0.16449340668482264365e-1_f64 * t96905 - t90525 + t90534 + t90542 - 0.49348022005446793095e-1_f64 * t96910 - t3758 * t28187 + t90547 - t96913 * t1386 - t56640 * t2016 - t90550 - t20044 * t6993;
    t96917
}
