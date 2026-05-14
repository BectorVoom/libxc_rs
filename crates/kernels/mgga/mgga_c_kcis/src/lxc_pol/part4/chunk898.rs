//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 898/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk898<F: Float>(t10269: F, t349: F, t1098: F, t3290: F, t3309: F, t3255: F, t3281: F, t245: F, t2840: F, t347: F, t313: F, t3262: F, t1035: F, t1103: F, t1018: F, t932: F) -> (F, F, F, F, F, F, F, F) {
    let t10271 = 0.29201909629629629629e-3 * t10269 * t349;
    let t10282 = t1098 * t3290;
    let t10284 = t1098 * t3309;
    let t10286 = t3255 * t3281;
    let t10292 = t2840 * t245 * t347;
    let t10297 = t3262 * t313;
    let t10314 = t1103 * t1035;
    let t10324 = t1018 * t932 * t347;
    (t10271, t10282, t10284, t10286, t10292, t10297, t10314, t10324)
}
