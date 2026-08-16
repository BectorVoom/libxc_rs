//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 441/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk441(t1651: f64, t407: f64, t1603: f64, t182: f64, t1170: f64, t119: f64, t1226: f64, t1228: f64, t1229: f64, t1231: f64, t1235: f64, t1238: f64, t1242: f64, t1246: f64, t151: f64, t1530: f64, t1627: f64, t1631: f64, t1633: f64, t1636: f64, t1639: f64, t1642: f64, t1645: f64, t1649: f64) -> (f64, f64, f64) {
    let t1652 = t1651 * t407;
    let t1655 = t182 * t1603;
    let t1658 = t1226 - t1228 - 0.65854491829355115987e0_f64 * t1229 + 0.65854491829355115987e0_f64 * t1231 + t1235 + 0.65854491829355115987e0_f64 * t1238 - 0.65854491829355115987e0_f64 * t1242 - t1246 - 0.65854491829355115987e0_f64 * t1627 + 0.65854491829355115987e0_f64 * t1631 + 0.13170898365871023197e1_f64 * t1530 * t1633 - 0.65854491829355115987e0_f64 * t151 * t1636 - 0.65854491829355115987e0_f64 * t151 * t1639 - 0.65854491829355115987e0_f64 * t1170 * t1642 + 0.65854491829355115987e0_f64 * t1645 - 0.65854491829355115987e0_f64 * t1649 - 0.65854491829355115987e0_f64 * t151 * t1652 + 0.65854491829355115987e0_f64 * t119 * t1655;
    (t1652, t1655, t1658)
}
