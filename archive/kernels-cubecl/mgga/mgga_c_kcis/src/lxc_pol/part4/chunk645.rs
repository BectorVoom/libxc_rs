//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 645/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk645<F: Float>(t1220: F, t3551: F, t2917: F, t2966: F, t2919: F, t2922: F, t2925: F, t2928: F, t2945: F, t2953: F, t2961: F, t2963: F, t2968: F, t2972: F, t2975: F, t2978: F) -> (F, F, F, F) {
    let t3552 = t3551 * t1220;
    let t3557 = F::cast_from(0.68863333333333333333e0_f64) * t2917;
    let t3564 = F::cast_from(0.17365833333333333333e0_f64) * t2966;
    let t3569 = -F::cast_from(0.17648625e1_f64) * t2945 + F::cast_from(0.3529725e1_f64) * t2953 + t3557 + F::cast_from(0.34431666666666666666e0_f64) * t2919 - F::cast_from(0.34431666666666666667e0_f64) * t2922 + F::cast_from(0.103295e1_f64) * t2925 - F::cast_from(0.516475e0_f64) * t2928 + F::cast_from(0.31558125e0_f64) * t2961 + F::cast_from(0.6311625e0_f64) * t2963 + t3564 + F::cast_from(0.13892666666666666667e0_f64) * t2968 - F::cast_from(0.34731666666666666667e-1_f64) * t2972 + F::cast_from(0.20839e0_f64) * t2975 - F::cast_from(0.104195e0_f64) * t2978;
    (t3552, t3557, t3564, t3569)
}
