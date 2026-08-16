//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 497/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk497(t1608: f64, t2104: f64, t286: f64, t1597: f64, t1599: f64, t2096: f64, t2100: f64, t619: f64, t1616: f64, t1592: f64, t1620: f64, t1949: f64, t1985: f64, t2004: f64, t2008: f64, t2014: f64, t2093: f64, t626: f64) -> (f64, f64, f64, f64, f64) {
    let t2105 = t1608 * t2104;
    let t2106 = t286 * t2105;
    let t2109 = -t2096 * t619 / 72.0_f64 + t1597 + t1599 * t2100 / 576.0_f64 - t1599 * t2106 / 192.0_f64;
    let t2110 = t2109 * t1616;
    let t2118 = t2093 * t626 - 0.66725e-1_f64 * t1592 * t2110 + t1620 + 0.11607361111111111111e-2_f64 * t1949 + 0.17411041666666666666e-2_f64 * t1985 - 0.17411041666666666666e-2_f64 * t2004 - 0.46429444444444444443e-2_f64 * t2008 + 0.11607361111111111111e-2_f64 * t2014;
    (t2105, t2106, t2109, t2110, t2118)
}
