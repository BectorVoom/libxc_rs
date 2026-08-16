//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 362/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk362(t1616: f64, t2109: f64, t1592: f64, t1620: f64, t1949: f64, t1985: f64, t2004: f64, t2008: f64, t2014: f64, t2093: f64, t626: f64, t2036: f64, t2040: f64, t2044: f64, t2048: f64, t2052: f64, t2056: f64, t2063: f64, t2067: f64) -> (f64, f64, f64) {
    let t2110 = t2109 * t1616;
    let t2118 = t2093 * t626 - 0.66725e-1_f64 * t1592 * t2110 + t1620 + 0.11607361111111111111e-2_f64 * t1949 + 0.17411041666666666666e-2_f64 * t1985 - 0.17411041666666666666e-2_f64 * t2004 - 0.46429444444444444443e-2_f64 * t2008 + 0.11607361111111111111e-2_f64 * t2014;
    let t2128 = 0.9375e-1_f64 * t2036 - 0.9375e-1_f64 * t2040 - 0.25e0_f64 * t2044 + 0.625e-1_f64 * t2048 - 0.101171875e-1_f64 * t2052 + 0.101171875e-1_f64 * t2056 + 0.53958333333333333333e-1_f64 * t2063 - 0.13489583333333333333e-1_f64 * t2067;
    (t2110, t2118, t2128)
}
