//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 823/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk823<F: Float>(t2855: F, t6613: F, t1096: F, t1092: F, t10463: F, t13102: F, t13103: F, t18461: F, t18465: F, t18468: F, t18471: F, t18474: F, t18477: F, t18483: F, t18486: F, t18495: F, t18498: F, t18500: F, t2836: F, t979: F) -> (F, F, F) {
    let t18502 = t2855 * t6613;
    let t18503 = t1096 * t18502;
    let t18504 = t1092 * t18503;
    let t18506 = -F::new(0.24872916666666666666e-2) * t18461 + F::new(0.16581944444444444444e-2) * t18465 + F::new(0.49745833333333333332e-2) * t18468 - t13102 + F::new(0.22109259259259259259e-2) * t13103 + F::new(0.22109259259259259259e-2) * t18471 - F::new(0.33163888888888888888e-2) * t18474 + F::new(0.890445125e-2) * t2836 * t18477 + F::new(0.66725e-1) * t979 * t18477 - F::new(0.178244852896875e-2) * t10463 * t18483 + F::new(0.178089025e-1) * t2836 * t18486 - F::new(0.13345e0) * t979 * t18483 - F::new(0.2671335375e-1) * t2836 * t18483 + F::new(0.13345e0) * t979 * t18486 + F::new(0.22109259259259259259e-2) * t18495 - F::new(0.88437037037037037035e-2) * t18498 - F::new(0.16581944444444444444e-2) * t18500 + F::new(0.66327777777777777776e-2) * t18504;
    (t18502, t18504, t18506)
}
