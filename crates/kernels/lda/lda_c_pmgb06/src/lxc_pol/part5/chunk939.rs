//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 939/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk939<F: Float>(t1822: F, t947: F, t1461: F, t2911: F, t2918: F, t495: F, t1464: F, t165: F, t1832: F, t8337: F, t1830: F, t839: F) -> (F, F, F, F, F, F, F) {
    let t13372 = t947 * t1822;
    let t13373 = F::new(0.0016792592592592592) * t13372;
    let t13384 = t1461 * t2911;
    let t13388 = t495 * t2918;
    let t13392 = t165 * t1464;
    let t13399 = t8337 * t1832;
    let t13407 = t1830 * t839;
    (t13372, t13373, t13384, t13388, t13392, t13399, t13407)
}
