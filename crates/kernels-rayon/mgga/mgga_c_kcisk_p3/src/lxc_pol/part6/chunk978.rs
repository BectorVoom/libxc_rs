//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 978/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk978(t1341: f64, t30226: f64, t13330: f64, t1411: f64, t19020: f64, t19028: f64, t25306: f64, t25327: f64, t25376: f64, t26755: f64, t30187: f64, t30192: f64, t30195: f64, t30198: f64, t30202: f64, t30208: f64, t30214: f64, t30218: f64, t30221: f64, t30224: f64) -> (f64, f64) {
    let t30227 = t1341 * t30226;
    let t30228 = t13330 * t30227;
    let t30229 = t1411 * t30228;
    let t30232 = 0.33163888888888888887e-2_f64 * t25306 - 0.66327777777777777776e-2_f64 * t25327 + 0.99491666666666666664e-2_f64 * t30187 + 0.55273148148148148145e-2_f64 * t30192 - 0.16581944444444444444e-2_f64 * t30195 + 0.1492375e-1_f64 * t30198 - 0.11054629629629629629e-2_f64 * t19020 + 0.49745833333333333332e-2_f64 * t30202 + 0.16581944444444444444e-2_f64 * t30208 + 0.99491666666666666664e-2_f64 * t25376 + 0.16581944444444444444e-2_f64 * t19028 - 0.49745833333333333332e-2_f64 * t30214 - 0.11054629629629629629e-2_f64 * t30218 - 0.74618749999999999998e-2_f64 * t30221 + 0.49745833333333333332e-2_f64 * t30224 - 0.1492375e-1_f64 * t30229 - 0.49745833333333333332e-2_f64 * t26755;
    (t30229, t30232)
}
