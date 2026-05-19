//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 965/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk965<F: Float>(t1046: F, t3054: F, t308: F, t9758: F, t1042: F, t2943: F, t9725: F, t3217: F, t982: F, t4585: F, t85: F, t349: F) -> (F, F, F, F, F, F, F) {
    let t10190 = t3054 * t1046;
    let t10199 = t9758 * t308;
    let t10202 = t2943 * t1042;
    let t10218 = F::cast_from(0.12841111111111111111e-1_f64) * t9725;
    let t10245 = t982 * t3217;
    let t10269 = t85 * t4585;
    let t10271 = F::cast_from(0.29201909629629629629e-3_f64) * t10269 * t349;
    (t10190, t10199, t10202, t10218, t10245, t10269, t10271)
}
