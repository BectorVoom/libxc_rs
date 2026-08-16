//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 853/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk853(t1378: f64, t20661: f64, t20594: f64, t562: f64, t1834: f64, t6361: f64, t1375: f64, t1843: f64, t20029: f64, t20044: f64, t20060: f64, t20420: f64, t20602: f64, t20609: f64, t20613: f64, t5215: f64, t5321: f64, t568: f64, t6440: f64, t6461: f64) -> (f64, f64, f64, f64) {
    let t20662 = t1378 * t20661;
    let t20670 = t20594 * t562;
    let t20672 = t6361 * t1834;
    let t20675 = -6.0_f64 * t1375 * t20609 + 6.0_f64 * t1375 * t20613 - t1375 * t20662 - 6.0_f64 * t1843 * t20029 - 3.0_f64 * t1843 * t20044 - 3.0_f64 * t1843 * t20060 + 3.0_f64 * t20420 * t568 + t20602 * t568 + t20670 * t568 + 3.0_f64 * t20672 * t568 + 6.0_f64 * t5215 * t6440 - 3.0_f64 * t5215 * t6461 + 6.0_f64 * t5321 * t6440 - 3.0_f64 * t5321 * t6461;
    (t20662, t20670, t20672, t20675)
}
