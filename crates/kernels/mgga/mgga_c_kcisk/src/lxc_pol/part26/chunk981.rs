//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 981/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk981<F: Float>(t1328: F, t26495: F, t1220: F, t19020: F, t19022: F, t19028: F, t19030: F, t19076: F, t25363: F, t25368: F, t25372: F, t25376: F, t25381: F, t25385: F, t25389: F, t25394: F, t25399: F, t25401: F, t25970: F, t25974: F, t3491: F, t8060: F) -> (F,) {
    let t26496 = t26495 * t1328;
    let t26501 = -0.24872916666666666666e-2 * t25363 - 0.33163888888888888888e-2 * t25368 + 0.18424382716049382715e-2 * t25372 - 0.7369753086419753086e-3 * t19020 - 0.58958024691358024688e-2 * t19022 + 0.33163888888888888888e-2 * t25376 + 0.11054629629629629629e-2 * t19028 + 0.88437037037037037035e-2 * t19030 + 0.88437037037037037035e-2 * t25381 + 0.66327777777777777776e-2 * t25385 + 0.14739506172839506173e-2 * t25389 - 0.33163888888888888888e-2 * t25394 - 0.66327777777777777776e-2 * t25399 + 0.22109259259259259258e-2 * t25401 + 0.24872916666666666666e-2 * t25970 - 0.13265555555555555555e-1 * t25974 - 0.193e0 * t1220 * t26496 - 0.193e0 * t3491 * t8060 + t19076;
    (t26501,)
}
