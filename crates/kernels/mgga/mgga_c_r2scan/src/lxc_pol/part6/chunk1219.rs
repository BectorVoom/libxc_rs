//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1219/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1219<F: Float>(t234: F, t5290: F, t5292: F, t712: F, t1696: F, t1831: F, t5358: F, t732: F, t21478: F, t225: F, t739: F, t5385: F, t741: F, t750: F, t1827: F, t1841: F, t5318: F) -> (F, F, F, F, F, F, F, F) {
    let t22500 = 0.49219290519438751956e5 * t234 * t5290 * t712 * t5292;
    let t22501 = t1696 * t1831;
    let t22505 = t732 * t5358;
    let t22512 = 0.11696447245269292414e1 * t234 * t739 * t225 * t21478;
    let t22513 = t5385 * t741;
    let t22515 = t5385 * t750;
    let t22517 = t1696 * t1827;
    let t22521 = 0.69263436422725855036e2 * t234 * t1841 * t5318;
    (t22500, t22501, t22505, t22512, t22513, t22515, t22517, t22521)
}
