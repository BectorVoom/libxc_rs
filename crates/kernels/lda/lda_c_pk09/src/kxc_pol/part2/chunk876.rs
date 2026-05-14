//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 876/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk876<F: Float>(t1504: F, t2594: F, t10020: F, t1319: F, t2507: F, t5267: F, t306: F, t1329: F, t309: F, t310: F, t2648: F, t5248: F, t1362: F, t1369: F, t2649: F, t623: F) -> (F, F, F, F, F, F, F) {
    let t10703 = t2594 * t1504;
    let t10706 = t1319 * t10020;
    let t10712 = t2507 * t5267;
    let t10713 = t10712 * t306;
    let t10715 = t309 * t310 * t1329;
    let t10719 = t2648 * t5248;
    let t10720 = t10719 * t1362;
    let t10721 = t310 * t10720;
    let t10724 = t1369 * t10020;
    let t10728 = t2649 * t623;
    (t10703, t10706, t10713, t10715, t10721, t10724, t10728)
}
