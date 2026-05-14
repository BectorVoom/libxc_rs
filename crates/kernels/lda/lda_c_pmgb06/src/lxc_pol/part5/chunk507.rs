//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 507/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk507<F: Float>(t1368: F, t2464: F, t2468: F, t2472: F, t2474: F, t2476: F, t2479: F, t2483: F, t2487: F, t2491: F, t2495: F, t183: F, t2414: F, t1374: F, t1379: F, t1412: F, t188: F, t2346: F, t2499: F, t2503: F, t2522: F, t2523: F, t2524: F, t2525: F) -> (F, F, F) {
    let t2675 = t2464 + t2468 + t2472 + t2474 + t2476 + t2479 + t2483 + t2487 - t2491 - t2495 + t1368;
    let t2676 = t2414 * t183;
    let t2680 = t1374 + t1379 - t2499 - t2503 + 4.0 / 3.0 * t2676 * t188 + t2522 + t2523 + t2524 + t2525 + 8.0 / 3.0 * t2346 + t1412;
    (t2675, t2676, t2680)
}
