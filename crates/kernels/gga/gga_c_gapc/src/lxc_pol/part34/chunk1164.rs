//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1164/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1164<F: Float>(t33405: F, t33409: F, t33413: F, t33417: F, t33420: F, t33407: F, t36570: F, t36571: F, t36572: F, t36573: F, t36574: F, t33427: F, t33436: F, t33441: F, t33454: F, t33457: F) -> (F, F, F, F, F, F) {
    let t36575 = 0.42205124476153752644e-7 * t33405;
    let t36577 = 0.10551281119038438161e-7 * t33409;
    let t36578 = 0.337303223138432284e-8 * t33413;
    let t36579 = 0.55331893559454114829e-8 * t33417;
    let t36580 = 0.66295654499063700026e-7 * t33420;
    let t36581 = -t36570 + t36571 - t36572 - t36573 + t36574 - t36575 - 0.3623181683912940217e-6 * t33407 + t36577 + t36578 + t36579 - t36580;
    let t36585 = 0.11372686522837130914e-5 * t33427;
    let t36587 = 0.18937162934584967535e-3 * t33436;
    let t36588 = 0.18937162934584967535e-3 * t33441;
    let t36593 = 0.91900712057578208105e-2 * t33454;
    let t36594 = 0.11594181388521408695e-4 * t33457;
    (t36581, t36585, t36587, t36588, t36593, t36594)
}
