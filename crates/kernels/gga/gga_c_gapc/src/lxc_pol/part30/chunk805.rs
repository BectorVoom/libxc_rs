//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 805/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk805<F: Float>(t3284: F, t7241: F, t1092: F, t2555: F, t191: F, t2786: F, t3304: F, t3278: F, t3285: F, t3289: F, t3288: F, t7178: F) -> (F, F, F, F, F, F, F) {
    let t9586 = t3284 * t7241;
    let t9587 = t1092 * t9586;
    let t9589 = t1092 * t2555;
    let t9591 = t2786 * t191;
    let t9592 = t9591 * t3304;
    let t9595 = t3278 * t3285;
    let t9597 = t3278 * t3289;
    let t9599 = t3288 * t7178;
    (t9586, t9587, t9589, t9592, t9595, t9597, t9599)
}
