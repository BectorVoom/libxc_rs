//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 997/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk997<F: Float>(t11997: F, t11998: F, t12011: F, t12028: F, t209: F, t3699: F, t501: F, t605: F, t1377: F, t3718: F, t1382: F, t12007: F, t549: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12030 = t11997 + t11998 + t12011 + t12028;
    let t12031 = t12030 * t209;
    let t12032 = t3699 * t501;
    let t12033 = t12032 * t605;
    let t12034 = t1377 * t3718;
    let t12035 = t3718 * t605;
    let t12036 = t1382 * t12035;
    let t12037 = F::new(2.0) * t12036;
    let t12038 = t549 * t12007;
    (t12030, t12031, t12032, t12033, t12034, t12035, t12036, t12037, t12038)
}
