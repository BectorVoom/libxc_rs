//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1136/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1136<F: Float>(t12140: F, t7064: F, t1368: F, t7053: F, t990: F, t3970: F, t7076: F, t3999: F, t7086: F, t1380: F, t613: F, t1315: F, t6948: F, t1336: F, t16115: F, t1907: F) -> (F, F, F, F, F, F) {
    let t21154 = t12140 * t7064;
    let t21155 = t1368 * t21154;
    let t21157 = t7053 * t990;
    let t21162 = t3970 * t7076;
    let t21163 = t1368 * t21162;
    let t21165 = t3999 * t7086;
    let t21166 = t21165 * t1380;
    let t21167 = t613 * t21166;
    let t21170 = t6948 * t1315;
    let t21172 = 1.0 * t21170 * t1336;
    let t21174 = 2.0 * t16115 * t1907;
    (t21155, t21157, t21163, t21167, t21172, t21174)
}
