//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 943/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk943<F: Float>(t2237: F, t27377: F, t27381: F, t27385: F, t27389: F, t27392: F, t27396: F, t27400: F, t27403: F, t27410: F, t7895: F, t7901: F, t7916: F, t4012: F, t7899: F, t6176: F) -> (F, F, F) {
    let t27413 = -0.15445601851851851852e-3 * t27377 + 0.16581944444444444444e-2 * t27381 + 0.27636574074074074073e-2 * t27385 - 0.33163888888888888888e-2 * t27389 - 0.24872916666666666666e-2 * t27392 - 0.13901041666666666667e-2 * t2237 * t27396 + 0.16581944444444444444e-2 * t27400 + 0.69505208333333333333e-3 * t2237 * t27403 + 0.13901041666666666667e-2 * t7895 * t7916 + 0.13901041666666666667e-2 * t7895 * t7901 + 0.18550940104166666667e-3 * t27410 * t7901;
    let t27415 = t7899 * t4012;
    let t27416 = t6176 * t27415;
    (t27413, t27415, t27416)
}
