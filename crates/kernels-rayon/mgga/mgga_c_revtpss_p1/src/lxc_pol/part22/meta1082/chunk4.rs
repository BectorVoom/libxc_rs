//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3905/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3905(t10073: f64, t22373: f64, t10069: f64, t22369: f64, t14216: f64, t14239: f64, t14220: f64, t48007: f64, t10130: f64, t46465: f64, t46490: f64, t48022: f64, t48027: f64, t48029: f64, t48036: f64, t48039: f64, t48041: f64, t6844: f64, t820: f64) -> f64 {
    let t74990 = t10073 * t22373;
    let t74999 = t10069 * t22369;
    let t75003 = t14239 * t14216;
    let t75005 = t48007 * t14220;
    let t75009 = 0.65049603595885220126e-3_f64 * t74990 + 0.65049603595885220126e-3_f64 * t46465 - 0.65854491829355115987e0_f64 * t820 * t10130 * t6844 - 0.43902994552903410656e-1_f64 * t48022 - 0.21951497276451705328e-1_f64 * t48027 - 0.520396828767081761e-2_f64 * t48029 - 0.73171657588172351096e-2_f64 * t74999 + 0.92526556154787137113e-2_f64 * t48036 + 0.52039682876708176102e-1_f64 * t48039 - 0.39029762157531132074e-1_f64 * t75003 - 0.23131639038696784277e-2_f64 * t75005 - 0.46263278077393568556e-2_f64 * t48041 + 0.13009920719177044025e-1_f64 * t46490;
    t75009
}
