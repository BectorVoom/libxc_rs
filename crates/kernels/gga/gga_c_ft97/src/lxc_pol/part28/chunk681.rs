//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 681/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk681<F: Float>(t428: F, t71: F, t420: F, t1301: F, t39: F, t5522: F, t1669: F, t22511: F, t5589: F, t53: F, t401: F, t22513: F, t22557: F, t22623: F, t32208: F, t32225: F, t32228: F, t32234: F, t32239: F, t32241: F, t32243: F, t32247: F, t32251: F, t32255: F, t32259: F, t52: F, t5613: F, t7181: F, t7182: F, t7195: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t32260 = t71 * t428;
    let t32261 = t420 * t32260;
    let t32262 = t1301 * t32261;
    let t32265 = t5522 * t39;
    let t32266 = t1669 * t32265;
    let t32267 = t22511 * t5589;
    let t32268 = t71 * t53;
    let t32269 = t420 * t32268;
    let t32270 = t32267 * t32269;
    let t32273 = t71 * t401;
    let t32274 = t420 * t32273;
    let t32278 = -0.44455354858818847408e-2 * t7181 * t52 * t7182 * t401 + 0.22227677429409423704e-2 * t22623 * t32225 - 0.22227677429409423704e-2 * t32228 * t32225 - 0.26350381008313446725e-3 * t79 * t32208 + 0.11352761063935582948e-3 * t22513 * t32234 - 0.68246728907663312894e-4 * t32239 * t32241 * t32243 - 0.25537443351851851852e-1 * t32247 * t5613 + 0.10338048737805743097e-3 * t32251 * t32255 + 0.15322466011111111111e0 * t32259 * t32262 - 0.18164417702296932716e-2 * t32266 * t32270 + 0.68116566383613497688e-3 * t22557 * t7195 * t32274;
    (t32260, t32261, t32266, t32267, t32268, t32273, t32278)
}
