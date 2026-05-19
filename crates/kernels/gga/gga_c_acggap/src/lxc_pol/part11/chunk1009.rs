//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1009/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1009<F: Float>(t30091: F, t525: F, t922: F, t1181: F, t30282: F, t599: F, t30090: F, t8952: F, t30123: F, t30151: F, t30085: F, t30089: F, t30094: F, t30099: F, t30106: F, t30110: F, t30118: F, t30121: F, t30125: F, t30130: F, t30132: F, t30139: F) -> (F, F) {
    let t33908 = F::cast_from(0.42874018118069736972e-3_f64) * t30091;
    let t33911 = t525 * t922;
    let t33914 = t30282 * t1181 * t599 * t33911;
    let t33916 = t30090 * t8952;
    let t33922 = F::cast_from(0.85748036236139473944e-3_f64) * t30123;
    let t33927 = F::cast_from(0.12579236915841660827e-2_f64) * t30151;
    let t33928 = F::cast_from(0.85748036236139473944e-3_f64) * t30085 + t30089 + t33908 + F::cast_from(0.21437009059034868486e-3_f64) * t30094 - F::cast_from(0.15724046144802076034e-3_f64) * t30099 - F::cast_from(0.32155513588552302729e-2_f64) * t33914 - F::cast_from(0.31448092289604152068e-3_f64) * t33916 + F::cast_from(0.37737710747524982481e-2_f64) * t30106 - F::cast_from(0.15724046144802076034e-3_f64) * t30110 + F::cast_from(0.10482697429868050689e-3_f64) * t30118 + F::cast_from(0.21437009059034868486e-3_f64) * t30121 + t33922 + F::cast_from(0.15724046144802076034e-3_f64) * t30125 - F::cast_from(0.20965394859736101378e-3_f64) * t30130 - F::cast_from(0.21437009059034868486e-3_f64) * t30132 - F::cast_from(0.14291339372689912324e-3_f64) * t30139 - t33927;
    (t33911, t33928)
}
