//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1146/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1146<F: Float>(t27370: F, t29288: F, t27369: F, t28336: F, t28369: F, t28392: F, t28395: F, t29259: F, t29267: F, t29271: F, t29275: F, t29278: F, t29281: F, t29284: F, t7908: F, t8144: F, t8148: F, t8155: F) -> (F, F) {
    let t29289 = t27370 * t29288;
    let t29296 = -F::new(0.15445601851851851852e-3) * t28336 + F::new(0.46336805555555555556e-3) * t7908 * t29259 - F::new(0.46336805555555555556e-3) * t28369 * t8155 + F::new(0.12356481481481481482e-2) * t28392 * t8155 + F::new(0.33163888888888888888e-2) * t29267 + F::new(0.16581944444444444444e-2) * t29271 + F::new(0.27636574074074074073e-2) * t29275 - F::new(0.33163888888888888888e-2) * t29278 + F::new(0.22109259259259259258e-2) * t29281 + F::new(0.61836467013888888889e-4) * t27369 * t29284 + F::new(0.22109259259259259258e-2) * t28395 - F::new(0.13901041666666666667e-2) * t7908 * t29289 - F::new(0.18550940104166666667e-3) * t27369 * t29289 + F::new(0.13901041666666666667e-2) * t8144 * t8148;
    (t29289, t29296)
}
