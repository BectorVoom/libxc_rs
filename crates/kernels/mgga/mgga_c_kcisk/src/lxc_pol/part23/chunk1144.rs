//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1144/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1144<F: Float>(t32203: F, t9462: F, t1339: F, t3283: F, t3797: F, t9461: F, t3278: F, t5634: F, t3759: F, t3579: F, t9447: F, t1312: F, t1405: F, t9474: F, t415: F, t32019: F, t32022: F, t32171: F, t32174: F, t32177: F, t32180: F, t32186: F, t32187: F, t32189: F, t32192: F, t32199: F, t32201: F, t9429: F, t9446: F, t9454: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32204 = t32203 * t9462;
    let t32205 = t1339 * t32204;
    let t32207 = t3797 * t3283;
    let t32208 = t9461 * t32207;
    let t32209 = t1339 * t32208;
    let t32211 = t5634 * t3278;
    let t32212 = t9461 * t32211;
    let t32213 = t3759 * t32212;
    let t32215 = t9447 * t3579;
    let t32216 = t1312 * t32215;
    let t32219 = t1405 * t9474;
    let t32220 = t415 * t32219;
    let t32222 = 0.24320185185185185185e-1 * t32171 + 0.69444444444444444446e-2 * t32174 + 0.69444444444444444446e-2 * t32177 + 0.10416666666666666667e-1 * t9446 * t32180 + 0.20833333333333333334e-1 * t32019 * t9454 - t32186 - 0.88437037037037037034e-2 * t32187 - 0.21444444444444444446e-1 * t32189 * t9429 + 0.26805555555555555556e-2 * t32192 - 0.55555555555555555558e-1 * t32022 * t9429 - 0.33163888888888888888e-2 * t32199 + 0.22109259259259259258e-2 * t32201 + 0.33163888888888888888e-2 * t32205 + 0.16581944444444444444e-2 * t32209 + 0.27636574074074074073e-2 * t32213 + 0.69444444444444444446e-2 * t9446 * t32216 - 0.13265555555555555555e-1 * t32220;
    (t32204, t32205, t32207, t32208, t32209, t32211, t32212, t32213, t32215, t32216, t32219, t32220, t32222)
}
