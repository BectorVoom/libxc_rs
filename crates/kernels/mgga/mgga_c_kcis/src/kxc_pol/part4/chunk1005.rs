//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1005/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1005<F: Float>(t1021: F, t14111: F, t1092: F, t10338: F, t1754: F, t2943: F, t304: F, t2944: F, t4601: F, t3255: F, t4603: F, t4608: F, t1071: F, t1114: F, t13791: F, t347: F, t4625: F) -> (F, F, F, F, F, F, F, F) {
    let t14112 = t1021 * t14111;
    let t14113 = t1092 * t14112;
    let t14115 = t10338 * t1754;
    let t14117 = t304 * t2943;
    let t14118 = t4601 * t2944;
    let t14119 = t14117 * t14118;
    let t14125 = 0.98556445e-3 * t3255 * t4603;
    let t14127 = 0.19711289e-2 * t3255 * t4608;
    let t14128 = t1114 * t1071;
    let t14129 = t14128 * t13791;
    let t14132 = t347 * t4625;
    (t14113, t14115, t14118, t14119, t14125, t14127, t14129, t14132)
}
