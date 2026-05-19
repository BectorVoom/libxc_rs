//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 978/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk978<F: Float>(t1341: F, t30226: F, t13330: F, t1411: F, t19020: F, t19028: F, t25306: F, t25327: F, t25376: F, t26755: F, t30187: F, t30192: F, t30195: F, t30198: F, t30202: F, t30208: F, t30214: F, t30218: F, t30221: F, t30224: F) -> (F, F) {
    let t30227 = t1341 * t30226;
    let t30228 = t13330 * t30227;
    let t30229 = t1411 * t30228;
    let t30232 = F::cast_from(0.33163888888888888887e-2_f64) * t25306 - F::cast_from(0.66327777777777777776e-2_f64) * t25327 + F::cast_from(0.99491666666666666664e-2_f64) * t30187 + F::cast_from(0.55273148148148148145e-2_f64) * t30192 - F::cast_from(0.16581944444444444444e-2_f64) * t30195 + F::new(0.1492375e-1) * t30198 - F::cast_from(0.11054629629629629629e-2_f64) * t19020 + F::cast_from(0.49745833333333333332e-2_f64) * t30202 + F::cast_from(0.16581944444444444444e-2_f64) * t30208 + F::cast_from(0.99491666666666666664e-2_f64) * t25376 + F::cast_from(0.16581944444444444444e-2_f64) * t19028 - F::cast_from(0.49745833333333333332e-2_f64) * t30214 - F::cast_from(0.11054629629629629629e-2_f64) * t30218 - F::cast_from(0.74618749999999999998e-2_f64) * t30221 + F::cast_from(0.49745833333333333332e-2_f64) * t30224 - F::new(0.1492375e-1) * t30229 - F::cast_from(0.49745833333333333332e-2_f64) * t26755;
    (t30229, t30232)
}
