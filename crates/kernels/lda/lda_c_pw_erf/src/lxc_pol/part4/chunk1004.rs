//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1004/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1004<F: Float>(t3416: F, t4933: F, t1318: F, t3899: F, t5316: F, t4646: F, t518: F, t4702: F, t1472: F, t5371: F, t10162: F, t2187: F, t519: F, t2151: F, t825: F, t571: F) -> (F, F, F, F, F, F, F, F) {
    let t12529 = t3416 * t4933;
    let t12532 = t1318 * t3899 * t5316;
    let t12536 = t4646 * t518;
    let t12541 = t4702 * t518;
    let t12546 = t1472 * t5371;
    let t12557 = t519 * t10162 * t2187;
    let t12571 = t2151 * t825;
    let t12572 = t571 * t12571;
    (t12529, t12532, t12536, t12541, t12546, t12557, t12571, t12572)
}
