//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 826/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk826(t22134: f64, t291: f64, t800: f64, t1208: f64, t17975: f64, t1701: f64, t3780: f64, t5284: f64, t4092: f64, t5261: f64, t1111: f64, t1198: f64, t1472: f64, t14729: f64, t14742: f64, t19039: f64, t19049: f64, t19135: f64, t22003: f64, t22013: f64, t22063: f64, t22065: f64, t22069: f64, t22071: f64, t22073: f64, t22078: f64, t22082: f64, t22085: f64, t22090: f64, t2691: f64, t4099: f64, t5003: f64, t5016: f64, t5234: f64, t5265: f64) -> (f64, f64, f64, f64, f64) {
    let t22135 = t291 * t22134;
    let t22136 = t800 * t22135;
    let t22138 = t17975 * t1208;
    let t22139 = t1701 * t22138;
    let t22143 = t1701 * t3780 * t5284;
    let t22154 = t4092 * t5261;
    let t22159 = 6.0_f64 * t22063 - 6.0_f64 * t22065 - 6.0_f64 * t22069 + 0.16294492281990603462e0_f64 * t22071 * t22073 - 0.43791161479435967991e1_f64 * t19135 * t22013 + 0.43791161479435967991e1_f64 * t19039 * t22078 - 0.3624548033042297868e1_f64 * t22082 * t1111 - 6.0_f64 * t2691 * t22085 - 0.72490960660845957359e1_f64 * t19049 * t22003 - 0.13867201135154723197e2_f64 * t5265 * t22090 * t291 + 2.0_f64 * t22136 + 0.11477735437967276582e2_f64 * t1472 * t22139 + 0.3624548033042297868e1_f64 * t14729 * t22143 - 0.3624548033042297868e1_f64 * t14742 * t22143 - 0.11477735437967276582e2_f64 * t4099 * t22139 - 0.22955470875934553164e2_f64 * t1198 * t5003 + 0.22955470875934553164e2_f64 * t5234 * t5003 - 0.3624548033042297868e1_f64 * t22154 * t1111 - 0.17516464591774387197e2_f64 * t5234 * t5016;
    (t22135, t22136, t22138, t22154, t22159)
}
