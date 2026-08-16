//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1732/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1732(t1065: f64, t3075: f64, t906: f64, t1042: f64, t1047: f64, t1063: f64, t1068: f64, t11977: f64, t11980: f64, t11983: f64, t11989: f64, t11991: f64, t11994: f64, t11999: f64, t12004: f64, t12007: f64, t12010: f64, t12013: f64, t12017: f64, t12021: f64, t3115: f64, t3127: f64, t3130: f64, t3157: f64, t3164: f64) -> (f64, f64, f64) {
    let t12024 = t1065 * t3075;
    let t12025 = t12024 * t906;
    let t12026 = t1042 * t12025;
    let t12029 = -0.68598428988911579154e-2_f64 * t11977 * t1047 + 0.85748036236139473944e-3_f64 * t11980 + 0.71456696863449561621e-3_f64 * t1063 * t11983 - 0.95275595817932748825e-4_f64 * t11989 + 0.42874018118069736972e-3_f64 * t11991 * t1068 - 0.85748036236139473944e-3_f64 * t11994 * t3130 + 0.34299214494455789577e-2_f64 * t11999 * t3164 + 0.14481890564325777821e-1_f64 * t12004 * t1068 - 0.30488190661738479624e-2_f64 * t12007 + 0.85748036236139473944e-3_f64 * t12010 - 0.68598428988911579154e-2_f64 * t12013 * t3157 - 0.64311027177104605458e-3_f64 * t3115 * t12017 + 0.64311027177104605458e-3_f64 * t12021 * t1047 - 0.42874018118069736972e-3_f64 * t3127 * t12026;
    (t12025, t12026, t12029)
}
