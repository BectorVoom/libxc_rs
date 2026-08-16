//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1009/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1009(t30091: f64, t525: f64, t922: f64, t1181: f64, t30282: f64, t599: f64, t30090: f64, t8952: f64, t30123: f64, t30151: f64, t30085: f64, t30089: f64, t30094: f64, t30099: f64, t30106: f64, t30110: f64, t30118: f64, t30121: f64, t30125: f64, t30130: f64, t30132: f64, t30139: f64) -> (f64, f64) {
    let t33908 = 0.42874018118069736972e-3_f64 * t30091;
    let t33911 = t525 * t922;
    let t33914 = t30282 * t1181 * t599 * t33911;
    let t33916 = t30090 * t8952;
    let t33922 = 0.85748036236139473944e-3_f64 * t30123;
    let t33927 = 0.12579236915841660827e-2_f64 * t30151;
    let t33928 = 0.85748036236139473944e-3_f64 * t30085 + t30089 + t33908 + 0.21437009059034868486e-3_f64 * t30094 - 0.15724046144802076034e-3_f64 * t30099 - 0.32155513588552302729e-2_f64 * t33914 - 0.31448092289604152068e-3_f64 * t33916 + 0.37737710747524982481e-2_f64 * t30106 - 0.15724046144802076034e-3_f64 * t30110 + 0.10482697429868050689e-3_f64 * t30118 + 0.21437009059034868486e-3_f64 * t30121 + t33922 + 0.15724046144802076034e-3_f64 * t30125 - 0.20965394859736101378e-3_f64 * t30130 - 0.21437009059034868486e-3_f64 * t30132 - 0.14291339372689912324e-3_f64 * t30139 - t33927;
    (t33911, t33928)
}
