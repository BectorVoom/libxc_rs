//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1298/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1298(t1165: f64, t1173: f64, t1181: f64, t1531: f64, t1532: f64, t1552: f64, t1759: f64, t18690: f64, t18702: f64, t18704: f64, t1899: f64, t20138: f64, t24110: f64, t24113: f64, t24128: f64, t24130: f64, t24138: f64, t3396: f64, t4267: f64, t4450: f64, t5116: f64, t839: f64, t945: f64) -> f64 {
    let t24141 = -0.34299214494455789578e-2_f64 * t1173 * t1165 * t1552 * t1759 * t839 + 0.68598428988911579156e-2_f64 * t24110 - 0.51448821741683684367e-2_f64 * t4450 * t1165 * t1532 * t24113 + 0.51448821741683684367e-2_f64 * t1531 * t1165 * t1532 * t20138 + 0.68026775414003982663e-1_f64 * t18690 + 0.17149607247227894789e-2_f64 * t18702 - 0.85748036236139473944e-3_f64 * t1531 * t1165 * t1899 * t945 - 0.34299214494455789578e-1_f64 * t24128 + 0.12004725073059526353e-1_f64 * t24130 + 0.13719685797782315831e-1_f64 * t3396 * t1181 * t4267 * t5116 - 0.85748036236139473944e-2_f64 * t24138 + 0.17149607247227894789e-2_f64 * t18704;
    t24141
}
