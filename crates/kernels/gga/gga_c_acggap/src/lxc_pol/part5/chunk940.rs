//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 940/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk940<F: Float>(t14255: F, t317: F, t863: F, t3883: F, t852: F, t13484: F, t13487: F, t180: F, t14401: F, t323: F, t1210: F, t851: F) -> (F, F, F, F, F) {
    let t14620 = F::new(0.39512695097613069591e1) * t863 * t317 * t14255;
    let t14621 = t852 * t3883;
    let t14626 = F::new(0.15805078039045227836e2) * t13484 * t180 * t317 * t13487;
    let t14640 = F::new(0.26341796731742046395e1) * t14401 * t180 * t323;
    let t14642 = t851 * t1210 * t323;
    (t14620, t14621, t14626, t14640, t14642)
}
