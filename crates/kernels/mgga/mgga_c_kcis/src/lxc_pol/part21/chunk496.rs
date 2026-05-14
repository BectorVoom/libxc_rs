//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 496/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk496<F: Float>(t1022: F, t3228: F, t3227: F, t1092: F, t1121: F, t2855: F, t1096: F, t2866: F, t359: F, t356: F, t303: F, t1087: F, t1126: F, t103: F, t251: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3229 = t1022 * t3228;
    let t3230 = t3227 * t3229;
    let t3231 = t1092 * t3230;
    let t3233 = t2855 * t1121;
    let t3234 = t1096 * t3233;
    let t3235 = t1092 * t3234;
    let t3237 = t2866 * t359;
    let t3238 = t356 * t3237;
    let t3239 = t303 * t3238;
    let t3241 = t1087 * t1126;
    let t3242 = t303 * t3241;
    let t3245 = t85 * t103 * t251;
    (t3229, t3230, t3231, t3233, t3234, t3235, t3237, t3238, t3239, t3241, t3242, t3245)
}
