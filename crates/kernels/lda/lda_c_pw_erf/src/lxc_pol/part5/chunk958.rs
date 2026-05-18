//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 958/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk958<F: Float>(t1401: F, t1484: F, t3476: F, t4500: F, t3704: F, t3964: F, t1621: F, t1931: F, t4233: F, t838: F, t10162: F, t2187: F, t519: F) -> (F, F, F, F, F, F) {
    let t12428 = t1484 * t1401;
    let t12439 = t4500 * t3476;
    let t12475 = t3964 * t3704;
    let t12507 = t1931 * t1621;
    let t12508 = F::new(4.0) * t12507;
    let t12509 = t838 * t4233;
    let t12557 = t519 * t10162 * t2187;
    (t12428, t12439, t12475, t12508, t12509, t12557)
}
