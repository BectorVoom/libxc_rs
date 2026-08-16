//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3000/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3000<F: Float>(t60359: F, t60371: F, t60374: F, t60377: F, t60381: F, t60384: F, t60387: F, t60919: F, t60923: F, t60930: F, t60932: F, t60936: F, t60938: F) -> F {
    let t62737 = -t60919 - t60923 - t60359 + t60930 + t60932 + t60936 - t60938 - t60371 - t60374 + t60377 + t60381 + t60384 + t60387;
    t62737
}
