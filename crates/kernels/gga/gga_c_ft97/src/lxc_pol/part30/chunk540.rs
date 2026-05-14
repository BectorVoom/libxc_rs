//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 540/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk540<F: Float>(t1501: F, t2842: F, t6362: F, t8392: F, t1882: F, t6371: F, t6300: F, t6349: F, t681: F, t89: F, t6304: F, t25035: F, t25153: F, t6388: F, t6386: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25271 = t2842 * t1501;
    let t25284 = t8392 * t6362;
    let t25298 = t1882 * t6371;
    let t25312 = t1882 * t6300;
    let t25315 = t89 * t681 * t6349;
    let t25317 = t1882 * t6304;
    let t25343 = 2.0 / 27.0 * t25035;
    let t25351 = 4.0 / 27.0 * t25153;
    let t25366 = t1882 * t6388;
    let t25368 = t870 * t6386;
    (t25271, t25284, t25298, t25312, t25315, t25317, t25343, t25351, t25366, t25368)
}
