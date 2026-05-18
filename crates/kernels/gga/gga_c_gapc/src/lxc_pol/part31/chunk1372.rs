//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1372/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1372<F: Float>(t33405: F, t33409: F, t33413: F, t33417: F, t33420: F, t33427: F, t33436: F, t33441: F, t33454: F, t33457: F, t33460: F, t33462: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36575 = F::new(0.42205124476153752644e-7) * t33405;
    let t36577 = F::new(0.10551281119038438161e-7) * t33409;
    let t36578 = F::new(0.337303223138432284e-8) * t33413;
    let t36579 = F::new(0.55331893559454114829e-8) * t33417;
    let t36580 = F::new(0.66295654499063700026e-7) * t33420;
    let t36585 = F::new(0.11372686522837130914e-5) * t33427;
    let t36587 = F::new(0.18937162934584967535e-3) * t33436;
    let t36588 = F::new(0.18937162934584967535e-3) * t33441;
    let t36593 = F::new(0.91900712057578208105e-2) * t33454;
    let t36594 = F::new(0.11594181388521408695e-4) * t33457;
    let t36596 = F::new(0.33764099580923002116e-6) * t33460;
    let t36597 = F::new(0.45018799441230669488e-6) * t33462;
    (t36575, t36577, t36578, t36579, t36580, t36585, t36587, t36588, t36593, t36594, t36596, t36597)
}
