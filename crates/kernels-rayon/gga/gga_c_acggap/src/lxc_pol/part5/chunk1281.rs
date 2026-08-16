//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1281/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1281(t1410: f64, t1539: f64, t1163: f64, t1165: f64, t1532: f64, t3379: f64, t5623: f64, t14174: f64, t18347: f64, t18349: f64, t18351: f64, t18364: f64, t18366: f64, t21532: f64, t23676: f64, t23680: f64, t23682: f64, t23686: f64, t3176: f64, t3403: f64) -> (f64, f64) {
    let t23688 = t1539 * t1410;
    let t23691 = t1163 * t1165 * t1532 * t23688;
    let t23697 = t3379 * t5623;
    let t23702 = 0.32012600194825403606e-1_f64 * t18347 + 0.25724410870841842184e-2_f64 * t23676 + 0.16006300097412701803e-1_f64 * t18349 + 0.80031500487063509016e-2_f64 * t18351 + 0.12004725073059526353e-1_f64 * t23680 - 0.56688979511669985553e-2_f64 * t23682 - 0.85748036236139473944e-3_f64 * t23686 + 0.85748036236139473944e-3_f64 * t23691 + 0.17149607247227894789e-1_f64 * t3403 * t1165 * t21532 * t3176 - 0.68598428988911579156e-2_f64 * t23697 + 0.34299214494455789578e-2_f64 * t14174 - 7.0_f64 / 36.0_f64 * t18364 - 7.0_f64 / 36.0_f64 * t18366;
    (t23688, t23702)
}
