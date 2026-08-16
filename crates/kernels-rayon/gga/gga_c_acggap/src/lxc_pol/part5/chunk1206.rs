//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1206/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1206(t3409: f64, t5895: f64, t12727: f64, t1755: f64, t3382: f64, t5792: f64, t1090: f64, t1165: f64, t1173: f64, t1180: f64, t1181: f64, t1444: f64, t1449: f64, t1531: f64, t1532: f64, t1552: f64, t1567: f64, t157: f64, t15947: f64, t16871: f64, t16940: f64, t16942: f64, t1884: f64, t1894: f64, t3169: f64, t3396: f64, t3403: f64, t372: f64, t4099: f64, t4463: f64, t6263: f64, t955: f64) -> f64 {
    let t21982 = t3409 * t5895;
    let t21994 = t12727 * t1755;
    let t22000 = t3382 * t5792;
    let t22011 = -0.17149607247227894789e-1_f64 * t3403 * t1181 * t1884 * t3169 - 0.34299214494455789578e-2_f64 * t1531 * t1165 * t1552 * t6263 * t372 + 0.40015750243531754508e-2_f64 * t21982 - 0.42874018118069736972e-3_f64 * t1180 * t1181 * t1894 * t955 + 0.45351183609335988442e-1_f64 * t16940 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t1532 * t157 * t4099 - 0.85748036236139473944e-3_f64 * t21994 + 0.34299214494455789578e-1_f64 * t4463 * t1181 * t1567 * t1444 - 0.17149607247227894789e-2_f64 * t22000 + 0.13719685797782315831e-1_f64 * t3396 * t1181 * t15947 * t1449 + 7.0_f64 / 144.0_f64 * t16942 - 0.10289764348336736873e0_f64 * t16871 * t1181 * t1884 * t1090;
    t22011
}
