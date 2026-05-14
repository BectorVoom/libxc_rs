//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 746/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk746<F: Float>(t2347: F, t743: F, t26: F, t666: F, t2360: F, t2567: F, t668: F, t2486: F, t754: F, t2372: F, t255: F, t9952: F, t258: F, t9570: F, t9577: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13683 = t743 * t2347;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13857 = t2567 * t668;
    let t13879 = t2486 * t754;
    let t13885 = t2372 * t255;
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14098 = t258 * t9577;
    let t14127 = t676 * t255;
    (t13683, t13688, t13689, t13857, t13879, t13885, t14080, t14081, t14098, t14127)
}
