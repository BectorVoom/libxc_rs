//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1181/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1181(t13263: f64, t1750: f64, t3379: f64, t6255: f64, t5891: f64, t1165: f64, t1180: f64, t13100: f64, t13110: f64, t13112: f64, t1531: f64, t1532: f64, t16602: f64, t16608: f64, t16610: f64, t335: f64, t336: f64, t5080: f64, t5630: f64, t5852: f64, t839: f64, t945: f64) -> f64 {
    let t21455 = t13263 * t1750;
    let t21457 = t3379 * t6255;
    let t21464 = t3379 * t5891;
    let t21467 = -t335 * t336 * t5630 * t839 / 48.0_f64 - 0.56688979511669985553e-2_f64 * t13100 + 0.85748036236139473945e-2_f64 * t13110 + 0.40015750243531754508e-1_f64 * t13112 - 0.42874018118069736972e-3_f64 * t1180 * t1165 * t1532 * t5080 - 0.80031500487063509016e-2_f64 * t16602 + 0.17149607247227894789e-2_f64 * t21455 + 0.34299214494455789578e-2_f64 * t21457 - 0.32012600194825403606e-1_f64 * t16608 + 0.30011812682648815881e-2_f64 * t1531 * t1165 * t5852 * t945 + 0.17149607247227894789e-2_f64 * t21464 - 0.80031500487063509016e-2_f64 * t16610;
    t21467
}
