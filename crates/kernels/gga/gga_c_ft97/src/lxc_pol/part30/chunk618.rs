//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 618/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk618<F: Float>(t263: F, t6837: F, t684: F, t2354: F, t10157: F, t3837: F, t6003: F, t1091: F, t24240: F, t24245: F, t1402: F, t3051: F) -> (F, F, F, F, F, F, F, F) {
    let t27991 = t6837 * t263;
    let t27992 = t27991 * t684;
    let t27993 = t2354 * t27992;
    let t27997 = t10157 * t6003 * t3837;
    let t28001 = t24240 * t1091;
    let t28002 = t2354 * t28001;
    let t28006 = t2354 * t24245 * t1091;
    let t28010 = t1402 * t3051;
    (t27991, t27992, t27993, t27997, t28001, t28002, t28006, t28010)
}
