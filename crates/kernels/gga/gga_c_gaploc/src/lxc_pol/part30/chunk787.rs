//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 787/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk787<F: Float>(t326: F, t7438: F, t1: F, t7284: F, t2021: F, t2717: F, t773: F, t2653: F, t783: F, t701: F, t7258: F, t1445: F) -> (F, F, F, F, F, F) {
    let t7439 = t326 * t7438;
    let t7442 = t7284 * t1;
    let t7443 = t2021 * t7442;
    let t7448 = t773 * t2717;
    let t7453 = t2653 * t783;
    let t7458 = t7258 * t701;
    let t7459 = t1445 * t7458;
    (t7439, t7442, t7443, t7448, t7453, t7459)
}
