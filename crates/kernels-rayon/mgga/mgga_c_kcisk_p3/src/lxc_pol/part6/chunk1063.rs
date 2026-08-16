//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1063/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1063(t31260: f64, t31404: f64, t1459: f64, t6591: f64, t8403: f64, t1557: f64, t19020: f64, t19028: f64, t25306: f64, t25327: f64, t25376: f64, t30187: f64, t30192: f64, t30195: f64, t30198: f64, t30202: f64, t30208: f64, t30214: f64, t30218: f64, t30221: f64, t30224: f64, t30229: f64) -> (f64, f64, f64) {
    let t31405 = t31260 + t31404;
    let t31406 = t1459 * t31405;
    let t31420 = t6591 * t8403;
    let t31426 = 0.23214722222222222222e-2_f64 * t25306 - 0.46429444444444444443e-2_f64 * t25327 + 0.69644166666666666666e-2_f64 * t30187 + 0.38691203703703703703e-2_f64 * t30192 - 0.11607361111111111111e-2_f64 * t30195 + 0.10446625e-1_f64 * t30198 - 0.77382407407407407405e-3_f64 * t19020 + 0.34822083333333333333e-2_f64 * t30202 + 0.11607361111111111111e-2_f64 * t30208 + 0.69644166666666666665e-2_f64 * t25376 + 0.11607361111111111111e-2_f64 * t19028 - 0.34822083333333333333e-2_f64 * t30214 - 0.77382407407407407405e-3_f64 * t30218 + 0.579e0_f64 * t1557 * t31420 - 0.52233124999999999998e-2_f64 * t30221 + 0.34822083333333333333e-2_f64 * t30224 - 0.10446625e-1_f64 * t30229;
    (t31406, t31420, t31426)
}
