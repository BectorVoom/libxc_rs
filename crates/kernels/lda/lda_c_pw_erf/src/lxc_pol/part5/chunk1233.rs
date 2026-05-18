//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1233/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1233<F: Float>(t1318: F, t1466: F, t2065: F, t6991: F, t518: F, t7660: F, t525: F, t18011: F, t4804: F, t7577: F, t3794: F, t2540: F, t5334: F) -> (F, F, F, F, F, F) {
    let t22204 = F::new(4.0) / F::new(5.0) * t1318 * t1466 * t6991 * t2065;
    let t22205 = t7660 * t518;
    let t22207 = F::new(4.0) / F::new(45.0) * t22205 * t525;
    let t22208 = F::new(8.0) / F::new(15.0) * t18011;
    let t22210 = F::new(8.0) / F::new(5.0) * t4804 * t7577;
    let t22212 = F::new(8.0) / F::new(5.0) * t3794 * t7577;
    let t22214 = F::new(4.0) / F::new(15.0) * t5334 * t2540;
    (t22204, t22207, t22208, t22210, t22212, t22214)
}
