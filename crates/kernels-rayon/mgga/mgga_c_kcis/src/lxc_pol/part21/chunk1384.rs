//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1384/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1384(t13093: f64, t2167: f64, t4527: f64, t7671: f64, t1658: f64, t18401: f64, t1876: f64, t2169: f64, t233: f64, t27150: f64, t2794: f64, t2801: f64, t28301: f64, t441: f64, t4534: f64, t7827: f64, t8027: f64, t911: f64, t91874: f64, t91885: f64, t91895: f64, t91901: f64, t92379: f64) -> f64 {
    let t97548 = t13093 * t2167;
    let t97561 = 2.0_f64 * t4527 * t7671;
    let t97567 = -t91874 + t97548 - t233 * t4534 * t7827 / 8.0_f64 - t2169 * t2801 * t1876 / 16.0_f64 - t2794 * t8027 / 8.0_f64 - t91885 - t2169 * t18401 * t441 / 16.0_f64 + t97561 + t91895 - t91901 - t233 * t1658 * t27150 / 16.0_f64 + t92379 + t911 * t28301 / 8.0_f64;
    t97567
}
