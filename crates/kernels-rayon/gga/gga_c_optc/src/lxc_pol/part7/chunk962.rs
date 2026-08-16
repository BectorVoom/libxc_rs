//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 962/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk962(t3245: f64, t9197: f64, t4289: f64, t9069: f64, t1162: f64, t1179: f64, t3234: f64, t3244: f64, t8953: f64, t8957: f64, t9169: f64, t9172: f64, t9175: f64, t9176: f64, t9179: f64, t9181: f64, t9188: f64, t9191: f64, t9194: f64, t9198: f64, t9202: f64, t9206: f64) -> f64 {
    let t9209 = t3245 * t9197;
    let t9212 = t4289 * t9069;
    let t9215 = 0.11983980782038291513e5_f64 * t9169 * t9172 - 0.8987985586528718635e4_f64 * t9175 * t9176 + 0.10076140891672839458e-1_f64 * t9179 + 0.16793568152788065762e-1_f64 * t9181 + 0.50380704458364197288e-2_f64 * t1179 * t8957 + 0.22391424203717421017e-1_f64 * t1179 * t8953 + t9188 + 0.1559479530529405812e2_f64 * t9191 - 0.30228422675018518374e-1_f64 * t1179 * t9194 + 0.1169609647897054359e2_f64 * t3234 * t9198 + 0.25190352229182098644e-1_f64 * t1179 * t9202 + 0.11590881986385010473e0_f64 * t1162 * t9206 + 0.11360101276506094136e1_f64 * t3244 * t9209 + 0.15146801702008125515e1_f64 * t3244 * t9212;
    t9215
}
