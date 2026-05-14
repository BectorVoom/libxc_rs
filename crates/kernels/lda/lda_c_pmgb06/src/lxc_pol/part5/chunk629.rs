//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 629/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk629<F: Float>(t5: F, t2395: F, t273: F, t698: F, t2377: F, t3912: F, t1068: F, t2381: F, t1072: F, t2125: F, t332: F, t5961: F, t9: F, t2386: F, t3922: F, t1079: F, t2389: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t6037 = t2395 * t273;
    let t6038 = t6037 * t698;
    let t6042 = t3912 * t2377;
    let t6047 = t1068 * t2381;
    let t6053 = piecewise3(t6, 0.0, -8.0 / 27.0 * t6042 * t332 + 16.0 / 9.0 * t2125 * t1072 + 4.0 / 9.0 * t6047 * t332 + 4.0 / 3.0 * t9 * t5961);
    let t6054 = t3922 * t2386;
    let t6059 = t1079 * t2389;
    (t6037, t6038, t6042, t6053, t6054, t6059)
}
