//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 869/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk869<F: Float>(t2075: F, t8251: F, t3484: F, t3482: F, t5886: F, t7907: F, t1411: F, t2236: F, t25308: F, t2231: F, t8072: F, t1341: F, t13330: F, t19020: F, t19028: F, t25306: F, t25327: F, t25376: F, t26755: F, t30187: F, t30192: F, t30195: F, t30198: F, t30202: F, t30208: F, t30214: F) -> (F, F, F, F, F, F) {
    let t30216 = t8251 * t2075;
    let t30217 = t3484 * t30216;
    let t30218 = t3482 * t30217;
    let t30220 = t5886 * t7907;
    let t30221 = t1411 * t30220;
    let t30223 = t25308 * t2236;
    let t30224 = t1411 * t30223;
    let t30226 = t8072 * t2231;
    let t30227 = t1341 * t30226;
    let t30228 = t13330 * t30227;
    let t30229 = t1411 * t30228;
    let t30232 = 0.33163888888888888887e-2 * t25306 - 0.66327777777777777776e-2 * t25327 + 0.99491666666666666664e-2 * t30187 + 0.55273148148148148145e-2 * t30192 - 0.16581944444444444444e-2 * t30195 + 0.1492375e-1 * t30198 - 0.11054629629629629629e-2 * t19020 + 0.49745833333333333332e-2 * t30202 + 0.16581944444444444444e-2 * t30208 + 0.99491666666666666664e-2 * t25376 + 0.16581944444444444444e-2 * t19028 - 0.49745833333333333332e-2 * t30214 - 0.11054629629629629629e-2 * t30218 - 0.74618749999999999998e-2 * t30221 + 0.49745833333333333332e-2 * t30224 - 0.1492375e-1 * t30229 - 0.49745833333333333332e-2 * t26755;
    (t30218, t30221, t30224, t30226, t30229, t30232)
}
