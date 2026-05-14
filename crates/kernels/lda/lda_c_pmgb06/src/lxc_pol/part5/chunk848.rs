//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 848/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk848<F: Float>(t13291: F, t3155: F, t831: F, t177: F, t2911: F, t2918: F, t1531: F, t1593: F, t1827: F, t947: F, t1822: F, t1461: F, t495: F, t1464: F, t165: F, t1832: F, t8337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13292 = t13291 / 45.0;
    let t13294 = t831 * t3155;
    let t13295 = t13294 / 45.0;
    let t13300 = t177 * t2911;
    let t13304 = t177 * t2918;
    let t13308 = t1593 * t1531;
    let t13370 = t947 * t1827;
    let t13372 = t947 * t1822;
    let t13373 = 0.0016792592592592592 * t13372;
    let t13384 = t1461 * t2911;
    let t13388 = t495 * t2918;
    let t13392 = t165 * t1464;
    let t13399 = t8337 * t1832;
    (t13292, t13295, t13300, t13304, t13308, t13370, t13372, t13373, t13384, t13388, t13392, t13399)
}
