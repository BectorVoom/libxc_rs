//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1177/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1177<F: Float>(t92039: F, t92294: F, t92300: F, t92302: F, t92305: F, t92307: F, t92310: F, t92312: F, t92314: F, t92316: F, t92319: F, t92321: F, t92325: F, t92327: F, t92329: F, t92332: F, t92334: F) -> F {
    let t92336 = F::cast_from(0.23425829475308641975e-1_f64) * t92294 - F::cast_from(0.10317654320987654321e0_f64) * t92039 + F::cast_from(0.41703125000000000001e-2_f64) * t92300 + F::cast_from(0.97307291666666666666e-2_f64) * t92302 + F::cast_from(0.1299702052373046875e-3_f64) * t92305 + F::cast_from(0.111403033060546875e-3_f64) * t92307 - F::cast_from(0.55701516530273437501e-4_f64) * t92310 - F::cast_from(0.41703125000000000001e-2_f64) * t92312 + F::cast_from(0.92754700520833333333e-4_f64) * t92314 + F::cast_from(0.10203017057291666667e-2_f64) * t92316 - F::cast_from(0.12985658072916666667e-2_f64) * t92319 - F::cast_from(0.64928290364583333333e-3_f64) * t92321 + F::cast_from(0.69505208333333333333e-3_f64) * t92325 + F::cast_from(0.208515625e-2_f64) * t92327 + F::cast_from(0.41703125000000000001e-2_f64) * t92329 + F::cast_from(0.2164276345486111111e-2_f64) * t92332 + F::cast_from(0.19478487109375e-2_f64) * t92334;
    t92336
}
