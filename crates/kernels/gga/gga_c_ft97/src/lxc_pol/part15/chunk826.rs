//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 826/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk826<F: Float>(t22134: F, t291: F, t800: F, t1208: F, t17975: F, t1701: F, t3780: F, t5284: F, t4092: F, t5261: F, t1111: F, t1198: F, t1472: F, t14729: F, t14742: F, t19039: F, t19049: F, t19135: F, t22003: F, t22013: F, t22063: F, t22065: F, t22069: F, t22071: F, t22073: F, t22078: F, t22082: F, t22085: F, t22090: F, t2691: F, t4099: F, t5003: F, t5016: F, t5234: F, t5265: F) -> (F, F, F, F, F) {
    let t22135 = t291 * t22134;
    let t22136 = t800 * t22135;
    let t22138 = t17975 * t1208;
    let t22139 = t1701 * t22138;
    let t22143 = t1701 * t3780 * t5284;
    let t22154 = t4092 * t5261;
    let t22159 = F::new(6.0) * t22063 - F::new(6.0) * t22065 - F::new(6.0) * t22069 + F::cast_from(0.16294492281990603462e0_f64) * t22071 * t22073 - F::cast_from(0.43791161479435967991e1_f64) * t19135 * t22013 + F::cast_from(0.43791161479435967991e1_f64) * t19039 * t22078 - F::cast_from(0.3624548033042297868e1_f64) * t22082 * t1111 - F::new(6.0) * t2691 * t22085 - F::cast_from(0.72490960660845957359e1_f64) * t19049 * t22003 - F::cast_from(0.13867201135154723197e2_f64) * t5265 * t22090 * t291 + F::new(2.0) * t22136 + F::cast_from(0.11477735437967276582e2_f64) * t1472 * t22139 + F::cast_from(0.3624548033042297868e1_f64) * t14729 * t22143 - F::cast_from(0.3624548033042297868e1_f64) * t14742 * t22143 - F::cast_from(0.11477735437967276582e2_f64) * t4099 * t22139 - F::cast_from(0.22955470875934553164e2_f64) * t1198 * t5003 + F::cast_from(0.22955470875934553164e2_f64) * t5234 * t5003 - F::cast_from(0.3624548033042297868e1_f64) * t22154 * t1111 - F::cast_from(0.17516464591774387197e2_f64) * t5234 * t5016;
    (t22135, t22136, t22138, t22154, t22159)
}
