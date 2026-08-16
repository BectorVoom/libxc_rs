//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 833/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk833(t29342: f64, t29359: f64, t1378: f64, t2091: f64, t3887: f64, t6460: f64, t1375: f64, t20029: f64, t20044: f64, t20060: f64, t2092: f64, t24156: f64, t24157: f64, t26361: f64, t26475: f64, t28207: f64, t28211: f64, t28214: f64, t28234: f64, t5215: f64, t5321: f64, t6440: f64, t6461: f64, t7194: f64, t7925: f64, t7937: f64) -> (f64, f64, f64, f64) {
    let t29360 = t29342 + t29359;
    let t29361 = t1378 * t29360;
    let t29372 = t3887 * t2091 * t6460;
    let t29375 = -2.0_f64 * t20029 * t2092 + 4.0_f64 * t5215 * t7925 - 0.16449340668482264365e-1_f64 * t28207 + 2.0_f64 * t7194 * t6440 - 0.3289868133696452873e-1_f64 * t28211 - 0.6579736267392905746e-1_f64 * t28214 - 0.76763589786250567036e-1_f64 * t26361 - t20044 * t2092 - t1375 * t29361 - t20060 * t2092 - 0.16449340668482264365e-1_f64 * t26475 - 2.0_f64 * t5215 * t7937 - 2.0_f64 * t5321 * t7937 + 0.3289868133696452873e-1_f64 * t28234 - t7194 * t6461 + t24156 + t24157 + 2.0_f64 * t1375 * t29372;
    (t29360, t29361, t29372, t29375)
}
