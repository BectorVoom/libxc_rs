//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1292/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1292(t1111: f64, t1181: f64, t22705: f64, t3391: f64, t17550: f64, t6324: f64, t1180: f64, t13286: f64, t13287: f64, t13364: f64, t14292: f64, t14297: f64, t14301: f64, t14309: f64, t14313: f64, t1754: f64, t17656: f64, t17972: f64, t18566: f64, t20987: f64, t525: f64, t6269: f64, t6394: f64, t8401: f64) -> f64 {
    let t23991 = t3391 * t1181 * t22705 * t1111;
    let t23994 = t17550 * t6324;
    let t23996 = t14292 - 0.12004725073059526352e-1_f64 * t14297 + t14301 - 0.22675591804667994221e-1_f64 * t14309 + 0.85748036236139473944e-3_f64 * t14313 + 0.34299214494455789578e-2_f64 * t17656 * t13287 * t8401 * t6394 - 0.13719685797782315831e-1_f64 * t13286 * t13287 * t525 * t20987 + 0.68598428988911579156e-2_f64 * t13286 * t13364 * t8401 * t6269 - 0.85748036236139473944e-3_f64 * t1180 * t17972 * t1754 + 0.17149607247227894789e-2_f64 * t23991 + 0.85748036236139473944e-3_f64 * t18566 + 0.16006300097412701803e0_f64 * t23994;
    t23996
}
