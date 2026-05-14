//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 896/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk896<F: Float>(t3237: F, t9189: F, t3234: F, t3151: F, t9044: F, t894: F, t2860: F, t3236: F, t3235: F, t3146: F, t3087: F, t914: F, t3245: F, t4289: F, t9069: F, t1162: F, t1179: F, t3244: F, t8953: F, t8957: F, t9169: F, t9172: F, t9175: F, t9176: F, t9179: F, t9181: F, t9188: F) -> (F, F, F, F, F) {
    let t9190 = t9189 * t3237;
    let t9191 = t3234 * t9190;
    let t9193 = t3151 * t9044;
    let t9194 = t894 * t9193;
    let t9197 = t2860 * t3236;
    let t9198 = t3235 * t9197;
    let t9201 = t3146 * t9044;
    let t9202 = t894 * t9201;
    let t9205 = t3087 * t9044;
    let t9206 = t914 * t9205;
    let t9209 = t3245 * t9197;
    let t9212 = t4289 * t9069;
    let t9215 = 0.11983980782038291513e5 * t9169 * t9172 - 0.8987985586528718635e4 * t9175 * t9176 + 0.10076140891672839458e-1 * t9179 + 0.16793568152788065762e-1 * t9181 + 0.50380704458364197288e-2 * t1179 * t8957 + 0.22391424203717421017e-1 * t1179 * t8953 + t9188 + 0.1559479530529405812e2 * t9191 - 0.30228422675018518374e-1 * t1179 * t9194 + 0.1169609647897054359e2 * t3234 * t9198 + 0.25190352229182098644e-1 * t1179 * t9202 + 0.11590881986385010473e0 * t1162 * t9206 + 0.11360101276506094136e1 * t3244 * t9209 + 0.15146801702008125515e1 * t3244 * t9212;
    (t9193, t9197, t9201, t9205, t9215)
}
