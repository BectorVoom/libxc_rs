//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1009/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1009<F: Float>(t30091: F, t525: F, t922: F, t1181: F, t30282: F, t599: F, t30090: F, t8952: F, t30123: F, t30151: F, t30085: F, t30089: F, t30094: F, t30099: F, t30106: F, t30110: F, t30118: F, t30121: F, t30125: F, t30130: F, t30132: F, t30139: F) -> (F, F) {
    let t33908 = F::new(0.42874018118069736972e-3) * t30091;
    let t33911 = t525 * t922;
    let t33914 = t30282 * t1181 * t599 * t33911;
    let t33916 = t30090 * t8952;
    let t33922 = F::new(0.85748036236139473944e-3) * t30123;
    let t33927 = F::new(0.12579236915841660827e-2) * t30151;
    let t33928 = F::new(0.85748036236139473944e-3) * t30085 + t30089 + t33908 + F::new(0.21437009059034868486e-3) * t30094 - F::new(0.15724046144802076034e-3) * t30099 - F::new(0.32155513588552302729e-2) * t33914 - F::new(0.31448092289604152068e-3) * t33916 + F::new(0.37737710747524982481e-2) * t30106 - F::new(0.15724046144802076034e-3) * t30110 + F::new(0.10482697429868050689e-3) * t30118 + F::new(0.21437009059034868486e-3) * t30121 + t33922 + F::new(0.15724046144802076034e-3) * t30125 - F::new(0.20965394859736101378e-3) * t30130 - F::new(0.21437009059034868486e-3) * t30132 - F::new(0.14291339372689912324e-3) * t30139 - t33927;
    (t33911, t33928)
}
