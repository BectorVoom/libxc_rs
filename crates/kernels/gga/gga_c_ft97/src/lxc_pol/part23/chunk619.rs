//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 619/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk619<F: Float>(t13753: F, t1882: F, t3714: F, t3692: F, t3696: F, t3701: F, t3951: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13754 = 2.0 / 9.0 * t13753;
    let t13780 = t1882 * t3714;
    let t13781 = 2.0 / 27.0 * t13780;
    let t13794 = t1882 * t3692;
    let t13795 = 4.0 / 81.0 * t13794;
    let t13809 = t1882 * t3696;
    let t13810 = 2.0 / 27.0 * t13809;
    let t13811 = t1882 * t3701;
    let t13812 = 4.0 / 27.0 * t13811;
    let t13830 = t3951 * t761;
    (t13754, t13780, t13781, t13794, t13795, t13809, t13810, t13811, t13812, t13830)
}
