//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 962/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk962<F: Float>(t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12948: F, t19100: F, t19102: F, t19106: F, t19116: F, t19121: F, t19125: F, t19129: F, t19484: F, t19485: F, t19488: F, t19491: F, t19494: F, t19497: F, t19526: F, t19528: F, t19554: F) -> (F,) {
    let t19556 = -t19484 + 0.36514074074074074074e-1 * t19485 - 0.27385555555555555556e-1 * t19488 + 0.16431333333333333333e0 * t19491 - 0.49293999999999999999e0 * t19494 - 0.36514074074074074075e-1 * t19497 + 0.11958666666666666667e1 * t19116 - 0.79724444444444444445e0 * t19121 - 0.19931111111111111111e0 * t19125 - 0.17938e1 * t19129 + t19526 + 0.15358125e0 * t19528 - 0.13287407407407407408e0 * t19100 + 0.21924222222222222222e1 * t19106 + 0.99655555555555555557e-1 * t12931 + 0.66437037037037037038e-1 * t12933 - 0.18257037037037037037e0 * t12935 + 0.54771111111111111111e-1 * t12937 + 0.18257037037037037037e-1 * t12939 - 0.19931111111111111111e0 * t12948 + 0.13287407407407407408e0 * t19102 + t19554;
    (t19556,)
}
