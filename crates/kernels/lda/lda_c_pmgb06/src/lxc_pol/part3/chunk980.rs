//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 980/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk980<F: Float>(t177: F, t2911: F, t12529: F, t12547: F, t2918: F, t5138: F, t1531: F, t1593: F, t12521: F, t5077: F, t13007: F, t5091: F, t12555: F, t5095: F, t13002: F, t5084: F) -> (F, F, F, F, F, F) {
    let t13300 = t177 * t2911;
    let t13303 = 8.0 / 27.0 * t12529 * t13300 * t12547;
    let t13304 = t177 * t2918;
    let t13307 = 2.0 / 3.0 * t5138 * t13304 * t12547;
    let t13308 = t1593 * t1531;
    let t13311 = 4.0 / 15.0 * t5077 * t13308 * t12521;
    let t13312 = t13007 * t5091;
    let t13313 = 8.0 / 45.0 * t13312;
    let t13314 = t12555 * t5095;
    let t13315 = 8.0 / 45.0 * t13314;
    let t13318 = 2.0 / 5.0 * t5077 * t5084 * t13002;
    (t13303, t13307, t13311, t13313, t13315, t13318)
}
