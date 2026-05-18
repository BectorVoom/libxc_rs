//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 598/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk598<F: Float>(t3700: F, t390: F, t1008: F, t1020: F, t1015: F, t144: F) -> (F, F, F) {
    let t3702 = F::new(0.17006693853500995666e-1) * t3700 * t390;
    let t3703 = t1008 * t1020;
    let t3706 = F::new(1.0) / t1015 / t144;
    (t3702, t3703, t3706)
}
