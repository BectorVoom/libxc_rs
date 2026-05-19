//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1198/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1198<F: Float>(t29277: F, t7064: F, t8970: F, t10752: F, t5288: F, t2558: F, t8844: F, t943: F, t2508: F, t25331: F, t2541: F, t25335: F, t7157: F) -> (F, F, F, F, F) {
    let t32258 = t7064 * t29277 * t8970;
    let t32259 = F::cast_from(0.1281754371690370714e-2_f64) * t32258;
    let t32266 = F::cast_from(0.46143157380853345702e-1_f64) * t5288 * t10752;
    let t32268 = t943 * t8844 * t2558;
    let t32269 = F::cast_from(0.32043859292259267849e-3_f64) * t32268;
    let t32272 = F::cast_from(0.11535789345213336425e0_f64) * t2508 * t2541 * t25331;
    let t32275 = F::cast_from(0.38452631150711121418e0_f64) * t2508 * t7157 * t25335;
    (t32259, t32266, t32269, t32272, t32275)
}
