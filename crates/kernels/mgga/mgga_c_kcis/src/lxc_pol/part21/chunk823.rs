//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 823/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk823<F: Float>(t3069: F, t331: F, t1027: F, t3097: F, t308: F, t9758: F, t1042: F, t2943: F, t3093: F, t932: F, t9725: F, t2861: F, t3184: F) -> (F, F, F, F, F, F, F) {
    let t10192 = t331 * t3069;
    let t10194 = t1027 * t3097;
    let t10199 = t9758 * t308;
    let t10202 = t2943 * t1042;
    let t10208 = t932 * t3093;
    let t10218 = F::cast_from(0.12841111111111111111e-1_f64) * t9725;
    let t10243 = t2861 * t3184;
    (t10192, t10194, t10199, t10202, t10208, t10218, t10243)
}
