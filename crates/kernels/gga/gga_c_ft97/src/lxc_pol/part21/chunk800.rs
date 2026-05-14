//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 800/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk800<F: Float>(t22632: F, t5829: F, t5830: F, t22643: F, t5824: F, t6: F, t8907: F, t8: F, t3392: F) -> (F, F, F, F, F) {
    let t23766 = t5829 * t22632 * t5830;
    let t23770 = t5824 * t22643;
    let t23772 = t8907 * t6;
    let t23773 = t23772 * t8;
    let t23774 = t3392 * t23773;
    (t23766, t23770, t23772, t23773, t23774)
}
