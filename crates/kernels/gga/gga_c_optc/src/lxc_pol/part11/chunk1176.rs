//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1176/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1176<F: Float>(t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F, t56978: F, t56981: F, t56984: F, t24863: F, t24864: F, t30189: F, t30270: F, t49378: F, t49381: F, t49385: F, t49387: F, t49393: F, t56988: F, t56991: F, t56994: F, t56997: F, t56999: F) -> (F, F) {
    let t57403 = -0.13772666666666666666e1 * t39411 - 0.91817777777777777776e0 * t39413 + 0.27545333333333333333e1 * t39418 + 0.13772666666666666667e1 * t49240 - 0.41318e1 * t49242 - 0.166712e1 * t49271 + 0.27785333333333333333e0 * t49273 + 0.123954e2 * t56966 - 0.34431666666666666667e1 * t56969 - 0.13892666666666666667e0 * t56972 - 0.27785333333333333334e0 * t56975 - 0.185931e2 * t56978 + 0.41318e1 * t56981 - 0.13772666666666666667e1 * t56984;
    let t57416 = -0.375102e1 * t56988 + 0.83356e0 * t56991 + 0.125034e1 * t56994 + 0.12349037037037037037e1 * t30189 + t24863 + t24864 - 0.94674375e0 * t56997 + 0.1262325e1 * t56999 + 0.12349037037037037037e0 * t49378 + 0.27785333333333333333e0 * t49381 + 0.21424148148148148148e1 * t30270 - 0.27545333333333333332e1 * t49385 + 0.41318e1 * t49387 + 0.68863333333333333332e0 * t49393;
    (t57403, t57416)
}
