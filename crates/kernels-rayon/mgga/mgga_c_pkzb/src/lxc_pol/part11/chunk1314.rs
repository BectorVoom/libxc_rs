//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1314/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1314(t10261: f64, t11205: f64, t179: f64, t19026: f64, t23278: f64, t27119: f64, t27122: f64, t27151: f64, t27153: f64, t27155: f64, t27175: f64, t27178: f64, t27181: f64, t27232: f64, t3026: f64, t31086: f64, t3174: f64, t3235: f64, t404: f64, t758: f64, t824: f64, t932: f64) -> f64 {
    let t31892 = 0.25724410870841842184e-2_f64 * t27119 + 0.85748036236139473944e-3_f64 * t27122 - 0.42874018118069736972e-3_f64 * t404 * t179 * t932 * t31086 - 3.0_f64 / 16.0_f64 * t3174 * t23278 * t11205 * t824 + 0.91464571985215438872e-2_f64 * t27151 - 0.91464571985215438872e-2_f64 * t27153 + 0.45732285992607719436e-2_f64 * t27155 - 11.0_f64 / 162.0_f64 * t27175 + 0.25724410870841842183e-2_f64 * t27178 - 0.25724410870841842183e-2_f64 * t27181 - 5.0_f64 / 1296.0_f64 * t19026 - 0.1543464652250510531e-1_f64 * t3235 * t758 * t10261 * t3026 - 0.85748036236139473944e-3_f64 * t27232;
    t31892
}
