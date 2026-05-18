//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 709/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk709<F: Float>(t12703: F, t27096: F, t27064: F, t23463: F, t925: F, t2210: F, t23470: F, t3420: F, t379: F, t6708: F, t13220: F, t6699: F) -> (F, F, F, F, F, F, F, F) {
    let t27239 = t12703 * t27096;
    let t27242 = t12703 * t27064;
    let t27245 = t23463 * t925;
    let t27246 = t2210 * t27245;
    let t27249 = t23470 * t3420;
    let t27252 = t6708 * t379;
    let t27253 = t13220 * t27252;
    let t27256 = t6699 * t379;
    (t27239, t27242, t27245, t27246, t27249, t27252, t27253, t27256)
}
