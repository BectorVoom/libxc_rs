//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1202/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1202<F: Float>(t28351: F, t75638: F, t28335: F, t28392: F, t16823: F, t5737: F, t1307: F, t21827: F, t5709: F, t21868: F, t491: F, t990: F, t1385: F, t58540: F, t101868: F, t16884: F, t27369: F, t3984: F, t59380: F, t59578: F, t7908: F, t7909: F, t7911: F, t97997: F, t98016: F) -> (F, F, F, F, F) {
    let t103063 = t28351 * t75638;
    let t103066 = t28392 * t28335;
    let t103069 = t28351 * t16823 * t5737;
    let t103073 = t5709 * t21827 * t1307;
    let t103078 = t21868 * t491 * t990;
    let t103083 = t28351 * t58540 * t1385;
    let t103095 = -0.2782641015625e-3 * t27369 * t103063 + 0.41188271604938271607e-3 * t103066 - 0.556528203125e-3 * t27369 * t103069 - 0.46336805555555555557e-3 * t7908 * t103073 - 0.22109259259259259259e-2 * t97997 - 0.23168402777777777778e-3 * t103078 * t7911 + 0.41188271604938271607e-3 * t98016 + 0.41703125000000000001e-2 * t7908 * t103083 - 0.33163888888888888888e-2 * t101868 + 0.46336805555555555556e-3 * t7908 * t3984 * t7909 * t59578 - 0.92673611111111111112e-3 * t7908 * t16884 * t7909 * t59380;
    (t103063, t103069, t103073, t103083, t103095)
}
