//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1079/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1079<F: Float>(t10657: F, t10661: F, t11286: F, t1227: F, t1309: F, t1316: F, t14633: F, t14640: F, t14642: F, t14646: F, t14648: F, t14656: F, t14694: F, t14746: F, t2180: F, t2258: F, t2276: F, t312: F, t329: F, t346: F, t3656: F, t388: F, t4358: F, t4405: F, t5731: F, t5903: F, t77: F, t790: F, t8065: F) -> (F,) {
    let t14752 = 18.0 * t2180 * t5731 * t1227 + 3.0 * t346 * t2258 * t1309 - 18.0 * t4358 * t14633 + 0.5945049527603057 * t10657 - 2.7743564462147594 * t10661 - t14640 - 0.0008717022455366076 * t14642 - 0.0008717022455366076 * t14646 + 9.0 * t1316 * t388 * t14648 + t346 * t790 * t3656 + 18.0 * t4405 * t2276 + 3.0 * t1316 * t388 * t14656 + 6.0 * t346 * t5903 * t8065 + (t14694 + t14746) * t312 + 3.0 * t329 * t77 * t11286;
    (t14752,)
}
