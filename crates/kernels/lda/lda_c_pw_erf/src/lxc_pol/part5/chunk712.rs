//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 712/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk712<F: Float>(t352: F, t6365: F, t2017: F, t571: F, t2411: F, t549: F, t1318: F, t593: F, t3832: F, t2334: F, t3604: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6366 = t6365 * t352;
    let t6367 = t2017 * t6366;
    let t6369 = F::new(4.0) / F::new(27.0) * t571 * t6367;
    let t6370 = t2411 * t549;
    let t6371 = t2017 * t6370;
    let t6373 = F::new(8.0) / F::new(27.0) * t1318 * t6371;
    let t6374 = t2411 * t593;
    let t6375 = t3832 * t6374;
    let t6377 = F::new(4.0) / F::new(27.0) * t571 * t6375;
    let t6378 = t3604 * t2334;
    let t6379 = t6378 * t352;
    (t6366, t6367, t6369, t6370, t6371, t6373, t6374, t6375, t6377, t6378, t6379)
}
