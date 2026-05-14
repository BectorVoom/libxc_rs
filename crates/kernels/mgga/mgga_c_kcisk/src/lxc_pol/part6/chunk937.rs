//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 937/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk937<F: Float>(t31260: F, t31404: F, t1459: F, t6591: F, t8403: F, t1557: F, t19020: F, t19028: F, t25306: F, t25327: F, t25376: F, t30187: F, t30192: F, t30195: F, t30198: F, t30202: F, t30208: F, t30214: F, t30218: F, t30221: F, t30224: F, t30229: F) -> (F, F, F) {
    let t31405 = t31260 + t31404;
    let t31406 = t1459 * t31405;
    let t31420 = t6591 * t8403;
    let t31426 = 0.23214722222222222222e-2 * t25306 - 0.46429444444444444443e-2 * t25327 + 0.69644166666666666666e-2 * t30187 + 0.38691203703703703703e-2 * t30192 - 0.11607361111111111111e-2 * t30195 + 0.10446625e-1 * t30198 - 0.77382407407407407405e-3 * t19020 + 0.34822083333333333333e-2 * t30202 + 0.11607361111111111111e-2 * t30208 + 0.69644166666666666665e-2 * t25376 + 0.11607361111111111111e-2 * t19028 - 0.34822083333333333333e-2 * t30214 - 0.77382407407407407405e-3 * t30218 + 0.579e0 * t1557 * t31420 - 0.52233124999999999998e-2 * t30221 + 0.34822083333333333333e-2 * t30224 - 0.10446625e-1 * t30229;
    (t31406, t31420, t31426)
}
