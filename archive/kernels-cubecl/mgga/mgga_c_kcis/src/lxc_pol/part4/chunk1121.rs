//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1121/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1121<F: Float>(t1071: F, t1670: F, t2630: F, t3269: F, t13480: F, t4565: F, t13475: F, t4579: F, t13511: F, t3255: F, t4597: F, t1035: F, t3293: F) -> (F, F, F, F, F, F) {
    let t14155 = t3269 * t1670 * t1071 * t2630;
    let t14158 = t4565 * t13480;
    let t14161 = t4579 * t13475;
    let t14164 = t4579 * t13511;
    let t14168 = F::cast_from(0.13140859333333333333e-2_f64) * t3255 * t4597;
    let t14170 = t3293 * t1035;
    (t14155, t14158, t14161, t14164, t14168, t14170)
}
