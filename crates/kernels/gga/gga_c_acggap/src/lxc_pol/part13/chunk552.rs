//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 552/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk552<F: Float>(t3697: F, t384: F, t3669: F, t381: F, t390: F, t1008: F, t1020: F, t1015: F, t144: F, t1077: F, t322: F, t368: F, t398: F) -> (F, F, F, F, F, F) {
    let t3699 = F::cast_from(0.21437009059034868486e-3_f64) * t384 * t3697;
    let t3700 = t381 * t3669;
    let t3702 = F::cast_from(0.17006693853500995666e-1_f64) * t3700 * t390;
    let t3703 = t1008 * t1020;
    let t3706 = F::cast_from(1.0_f64) / t1015 / t144;
    let t3730 = t1077 * t322;
    let t3732 = t398 * t368 * t3730;
    (t3699, t3702, t3703, t3706, t3730, t3732)
}
