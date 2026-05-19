//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 962/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk962<F: Float>(t3245: F, t9197: F, t4289: F, t9069: F, t1162: F, t1179: F, t3234: F, t3244: F, t8953: F, t8957: F, t9169: F, t9172: F, t9175: F, t9176: F, t9179: F, t9181: F, t9188: F, t9191: F, t9194: F, t9198: F, t9202: F, t9206: F) -> F {
    let t9209 = t3245 * t9197;
    let t9212 = t4289 * t9069;
    let t9215 = F::cast_from(0.11983980782038291513e5_f64) * t9169 * t9172 - F::cast_from(0.8987985586528718635e4_f64) * t9175 * t9176 + F::cast_from(0.10076140891672839458e-1_f64) * t9179 + F::cast_from(0.16793568152788065762e-1_f64) * t9181 + F::cast_from(0.50380704458364197288e-2_f64) * t1179 * t8957 + F::cast_from(0.22391424203717421017e-1_f64) * t1179 * t8953 + t9188 + F::cast_from(0.1559479530529405812e2_f64) * t9191 - F::cast_from(0.30228422675018518374e-1_f64) * t1179 * t9194 + F::cast_from(0.1169609647897054359e2_f64) * t3234 * t9198 + F::cast_from(0.25190352229182098644e-1_f64) * t1179 * t9202 + F::cast_from(0.11590881986385010473e0_f64) * t1162 * t9206 + F::cast_from(0.11360101276506094136e1_f64) * t3244 * t9209 + F::cast_from(0.15146801702008125515e1_f64) * t3244 * t9212;
    t9215
}
