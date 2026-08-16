//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1042/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1042(t34009: f64, t34033: f64, t34039: f64, t34043: f64, t34056: f64, t34068: f64, t34127: f64, t34156: f64, t34179: f64, t34237: f64, t34255: f64, t34271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36898 = 0.42874018118069736972e-3_f64 * t34009;
    let t36911 = 0.21437009059034868486e-3_f64 * t34033;
    let t36914 = 0.28582678745379824648e-3_f64 * t34039;
    let t36916 = 0.38110238327173099531e-2_f64 * t34043;
    let t36920 = 0.14291339372689912324e-2_f64 * t34056;
    let t36925 = 0.85748036236139473944e-3_f64 * t34068;
    let t36950 = 0.28582678745379824648e-3_f64 * t34127;
    let t36961 = 0.18868855373762491241e-2_f64 * t34156;
    let t36970 = 0.20965394859736101378e-2_f64 * t34179;
    let t36993 = 0.42874018118069736972e-3_f64 * t34237;
    let t36998 = 0.85748036236139473944e-3_f64 * t34255;
    let t37003 = 0.17149607247227894789e-2_f64 * t34271;
    (t36898, t36911, t36914, t36916, t36920, t36925, t36950, t36961, t36970, t36993, t36998, t37003)
}
