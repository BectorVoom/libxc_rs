//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 908/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk908<F: Float>(t25342: F, t5634: F, t3484: F, t3482: F, t1451: F, t25308: F, t1411: F, t1413: F, t8161: F, t1441: F, t3512: F, t8082: F, t19861: F, t2236: F, t25294: F, t25299: F, t25304: F, t25306: F, t25310: F, t25316: F, t25319: F, t25322: F, t25325: F, t25327: F, t25331: F, t25335: F, t25340: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25343 = t5634 * t25342;
    let t25344 = t3484 * t25343;
    let t25345 = t3482 * t25344;
    let t25347 = t25308 * t1451;
    let t25348 = t1411 * t25347;
    let t25350 = t8161 * t1413;
    let t25351 = t25350 * sigma0;
    let t25352 = t25351 * t1441;
    let t25353 = t1411 * t25352;
    let t25355 = t3512 * t8082;
    let t25356 = t1411 * t25355;
    let t25358 = t19861 * t2236;
    let t25359 = t1411 * t25358;
    let t25361 = -0.33163888888888888888e-2 * t25294 + 0.33163888888888888888e-2 * t25299 + 0.99491666666666666664e-2 * t25304 + 0.11054629629629629629e-2 * t25306 + 0.16581944444444444444e-2 * t25310 + 0.16581944444444444444e-2 * t25316 + 0.27636574074074074073e-2 * t25319 + 0.66327777777777777776e-2 * t25322 + 0.16581944444444444444e-2 * t25325 - 0.22109259259259259259e-2 * t25327 + 0.66327777777777777776e-2 * t25331 + 0.88437037037037037035e-2 * t25335 - 0.22109259259259259259e-2 * t25340 + 0.99491666666666666664e-2 * t25345 + 0.16581944444444444444e-2 * t25348 - 0.24872916666666666666e-2 * t25353 - 0.55273148148148148147e-3 * t25356 + 0.33163888888888888888e-2 * t25359;
    (t25343, t25345, t25348, t25350, t25351, t25353, t25356, t25359, t25361)
}
