//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1878/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1878(t20986: f64, t4180: f64, t4181: f64, t119: f64, t20800: f64, t210: f64, t13251: f64, t16940: f64, t20963: f64, t20969: f64, t20974: f64, t20978: f64, t20983: f64, t2630: f64, t2643: f64, t4167: f64, t4178: f64, t5593: f64, t5614: f64, t5619: f64, t787: f64, t817: f64) -> (f64, f64, f64, f64) {
    let t20988 = t4180 * t4181 * t20986;
    let t20993 = t119 * t20800;
    let t20994 = t210 * t20993;
    let t20998 = -t4167 * t5614 / 1024.0_f64 + t2630 * t20963 / 512.0_f64 - t4167 * t5619 / 1024.0_f64 - t817 * t20969 / 3072.0_f64 - 5.0_f64 / 256.0_f64 * t2643 * t20974 + t2643 * t20978 / 256.0_f64 - t4178 * t20983 / 128.0_f64 + t4178 * t20988 / 512.0_f64 + t13251 * t5593 / 128.0_f64 - t787 * t20994 / 48.0_f64 + 7.0_f64 / 1536.0_f64 * t16940;
    (t20988, t20993, t20994, t20998)
}
