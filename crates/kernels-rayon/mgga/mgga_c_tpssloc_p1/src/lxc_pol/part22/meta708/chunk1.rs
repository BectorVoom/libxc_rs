//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2303/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2303(t67064: f64, t67082: f64, t157: f64, t182: f64, t46130: f64, t57887: f64, t46132: f64, t46134: f64, t57897: f64, t40667: f64, t40682: f64, t172: f64, t20742: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67083 = t67064 + t67082;
    let t67086 = 0.19751673498613801407e-1_f64 * t67083 * t157 * t182;
    let t67087 = 0.15584273195113317383e3_f64 * t46130;
    let t67088 = 3.0_f64 * t57887;
    let t67089 = 0.97592231702715658578e-1_f64 * t46132;
    let t67090 = 0.14447919941302971323e1_f64 * t46134;
    let t67095 = 3.0_f64 * t57897;
    let t67096 = 0.51947577317044391277e2_f64 * t40667;
    let t67097 = 0.35089341735807877242e1_f64 * t40682;
    let t67099 = t20742 * t172 * t763;
    (t67083, t67086, t67087, t67088, t67089, t67090, t67095, t67096, t67097, t67099)
}
