//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 652/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk652<F: Float>(t14489: F, t14490: F, t13015: F, t13018: F, t13026: F, t13028: F, t13036: F, t13040: F, t13044: F, t13047: F, t13050: F, t13849: F, t13852: F, t13855: F, t12661: F, t12665: F, t12667: F, t13054: F, t13057: F, t13059: F, t13859: F, t13863: F, t13867: F, t13874: F, t13878: F, t13882: F) -> (F, F, F) {
    let t14491 = t14489 + t14490;
    let t14498 = -t13015 - t13018 + t13026 + t13028 + t13036 - t13040 + t13044 - t13047 + 0.38342925953920749676e0 * t13849 - 0.38342925953920749676e0 * t13852 + t13855 - t13050;
    let t14506 = -0.76685851907841499353e0 * t12661 + t13054 - t13057 - 0.76685851907841499352e0 * t13059 - 0.92023022289409799224e1 * t13859 + 0.23005755572352449806e2 * t13863 - 0.13803453343411469884e2 * t13867 + 0.59584149919750711115e-1 * t12665 - 0.89376224879626066675e-1 * t12667 - t13874 + t13878 + t13882;
    (t14491, t14498, t14506)
}
