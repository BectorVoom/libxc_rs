//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 931/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk931(t5782: f64, t5786: f64, t118: f64, t1310: f64, t1315: f64, t1453: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t2322: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t508: f64, t511: f64, t5517: f64, t5528: f64, t569: f64, t649: f64, t651: f64, t671: f64) -> (f64, f64) {
    let t5787 = t5782 + t5786;
    let t5789 = -t118 * t5517 - t1310 * t1502 + t1315 * t1911 + t1453 * t1847 - 2.0_f64 * t1519 * t2322 - 2.0_f64 * t1519 * t4254 - t1843 * t649 - t4246 * t508 - 2.0_f64 * t4248 * t671 - 2.0_f64 * t4257 * t651 - 2.0_f64 * t4293 * t651 - 2.0_f64 * t4297 * t651 + t511 * t5787 + t5528 * t569;
    (t5787, t5789)
}
