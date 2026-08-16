//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 751/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk751(t428: f64, t71: f64, t420: f64, t1301: f64, t39: f64, t5522: f64, t1669: f64, t22511: f64, t5589: f64, t53: f64, t401: f64, t22513: f64, t22557: f64, t22623: f64, t32208: f64, t32225: f64, t32228: f64, t32234: f64, t32239: f64, t32241: f64, t32243: f64, t32247: f64, t32251: f64, t32255: f64, t32259: f64, t52: f64, t5613: f64, t7181: f64, t7182: f64, t7195: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
    let t32278 = -0.44455354858818847408e-2_f64 * t7181 * t52 * t7182 * t401 + 0.22227677429409423704e-2_f64 * t22623 * t32225 - 0.22227677429409423704e-2_f64 * t32228 * t32225 - 0.26350381008313446725e-3_f64 * t79 * t32208 + 0.11352761063935582948e-3_f64 * t22513 * t32234 - 0.68246728907663312894e-4_f64 * t32239 * t32241 * t32243 - 0.25537443351851851852e-1_f64 * t32247 * t5613 + 0.10338048737805743097e-3_f64 * t32251 * t32255 + 0.15322466011111111111e0_f64 * t32259 * t32262 - 0.18164417702296932716e-2_f64 * t32266 * t32270 + 0.68116566383613497688e-3_f64 * t22557 * t7195 * t32274;
    (t32260, t32261, t32266, t32267, t32268, t32273, t32278)
}
