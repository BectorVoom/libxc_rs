//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 835/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk835<F: Float>(t2611: F, t2617: F, t2607: F, t288: F, t656: F, t668: F, t912: F, t60: F, t721: F, t2663: F, t2738: F, t244: F, t2868: F) -> (F, F, F, F, F, F) {
    let t11661 = t2617 * t2611;
    let t11665 = F::new(0.67471172535210825684e-1) * t656 * t2607 * t288;
    let t11668 = F::new(0.86748650402413918736e-1) * t656 * t668 * t912;
    let t11669 = t60 * t721;
    let t11672 = F::new(0.1301229756036208781e0) * t11669 * t2738 * t2663;
    let t11679 = t2868 * t244;
    (t11661, t11665, t11668, t11669, t11672, t11679)
}
