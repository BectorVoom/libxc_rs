//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 893/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk893<F: Float>(t9725: F, t3217: F, t982: F, t4585: F, t85: F, t349: F, t245: F, t2840: F, t347: F, t313: F, t3262: F, t1035: F, t1103: F, t1018: F, t932: F, t3250: F, t41: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10218 = 0.12841111111111111111e-1 * t9725;
    let t10245 = t982 * t3217;
    let t10269 = t85 * t4585;
    let t10271 = 0.29201909629629629629e-3 * t10269 * t349;
    let t10292 = t2840 * t245 * t347;
    let t10297 = t3262 * t313;
    let t10314 = t1103 * t1035;
    let t10324 = t1018 * t932 * t347;
    let t10338 = t85 * t3250 * t41;
    (t10218, t10245, t10269, t10271, t10292, t10297, t10314, t10324, t10338)
}
