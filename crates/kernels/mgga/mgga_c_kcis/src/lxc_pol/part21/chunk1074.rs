//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1074/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1074<F: Float>(t2398: F, t26477: F, t8944: F, t26430: F, t7647: F, t7639: F, t26490: F, t7642: F, t26459: F, t26494: F, t26508: F, t36936: F, t695: F, t92066: F, t92068: F, t92070: F, t92072: F, t92074: F, t92076: F, t92078: F, t92080: F, t92082: F, t92086: F) -> (F,) {
    let t92089 = t8944 * t2398 * t26477;
    let t92091 = t26430 * t7647;
    let t92093 = t26430 * t7639;
    let t92095 = t7642 * t26490;
    let t92097 = t26459 * t7647;
    let t92099 = t26508 * t26494;
    let t92102 = t36936 * t695 * t26477;
    let t92104 = -0.12985658072916666667e-2 * t92066 + 0.208515625e-2 * t92068 - 0.97307291666666666666e-2 * t92070 - 0.97307291666666666666e-2 * t92072 - 0.41703125000000000001e-2 * t92074 + 0.10203017057291666667e-2 * t92076 - 0.41703125000000000001e-2 * t92078 + 0.69505208333333333333e-3 * t92080 - 0.48653645833333333332e-2 * t92082 + 0.69505208333333333333e-3 * t92086 + 0.1299702052373046875e-3 * t92089 + 0.16217881944444444444e-1 * t92091 + 0.16217881944444444444e-1 * t92093 - 0.48653645833333333332e-2 * t92095 + 0.208515625e-2 * t92097 + 0.2782641015625e-3 * t92099 - 0.55701516530273437501e-4 * t92102;
    (t92104,)
}
