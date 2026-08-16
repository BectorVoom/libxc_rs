//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1372/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1372(t33405: f64, t33409: f64, t33413: f64, t33417: f64, t33420: f64, t33427: f64, t33436: f64, t33441: f64, t33454: f64, t33457: f64, t33460: f64, t33462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36575 = 0.42205124476153752644e-7_f64 * t33405;
    let t36577 = 0.10551281119038438161e-7_f64 * t33409;
    let t36578 = 0.337303223138432284e-8_f64 * t33413;
    let t36579 = 0.55331893559454114829e-8_f64 * t33417;
    let t36580 = 0.66295654499063700026e-7_f64 * t33420;
    let t36585 = 0.11372686522837130914e-5_f64 * t33427;
    let t36587 = 0.18937162934584967535e-3_f64 * t33436;
    let t36588 = 0.18937162934584967535e-3_f64 * t33441;
    let t36593 = 0.91900712057578208105e-2_f64 * t33454;
    let t36594 = 0.11594181388521408695e-4_f64 * t33457;
    let t36596 = 0.33764099580923002116e-6_f64 * t33460;
    let t36597 = 0.45018799441230669488e-6_f64 * t33462;
    (t36575, t36577, t36578, t36579, t36580, t36585, t36587, t36588, t36593, t36594, t36596, t36597)
}
