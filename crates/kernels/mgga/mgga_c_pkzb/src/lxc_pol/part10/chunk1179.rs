//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1179/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1179<F: Float>(t2363: F, t2421: F, t6417: F, t7832: F, t2393: F, t2463: F, t23: F, t4810: F, t1424: F, t434: F, t4794: F, t7: F, t2493: F, t500: F, t2503: F, t1448: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19251 = t2363 * t2421;
    let t19283 = t7832 * t6417;
    let t19297 = t2393 * t2421;
    let t19338 = t2463 * t2463;
    let t19339 = 1.0 / t19338;
    let t19377 = t23 * t4810;
    let t19393 = t434 * t1424;
    let t19396 = t7 * t4794;
    let t19442 = t2493 * t500;
    let t19444 = 20.0 * t7 * t19442;
    let t19453 = t2503 * t500;
    let t19455 = 20.0 * t23 * t19453;
    let t19467 = t448 * t1448;
    (t19251, t19283, t19297, t19339, t19377, t19393, t19396, t19442, t19444, t19453, t19455, t19467)
}
