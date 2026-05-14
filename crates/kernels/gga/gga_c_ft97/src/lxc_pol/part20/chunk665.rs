//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 665/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk665<F: Float>(t13309: F, t4334: F, t2440: F, t327: F, t13315: F, t13320: F, t13352: F, t10864: F, t1091: F, t2939: F, t13346: F, t4342: F, t1268: F, t2349: F, t2360: F, t2923: F) -> (F, F, F, F, F, F, F) {
    let t14484 = t4334 * t13309;
    let t14487 = t2440 * t327;
    let t14488 = t14487 * t13315;
    let t14491 = t4334 * t13320;
    let t14497 = t4334 * t13352;
    let t14501 = t10864 * t1091 * t2939;
    let t14503 = t4342 * t13346;
    let t14507 = t2923 * t1268 * t2360 * t2349;
    (t14484, t14488, t14491, t14497, t14501, t14503, t14507)
}
