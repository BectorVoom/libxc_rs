//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1173/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1173<F: Float>(t21397: F, t494: F, t3965: F, t4494: F, t12314: F, t6756: F, t6492: F, t6762: F, t6352: F, t6766: F, t348: F, t12439: F, t4488: F) -> (F, F, F, F, F, F, F) {
    let t21398 = t21397 * t494;
    let t21401 = F::new(16.0) / F::new(15.0) * t3965 * t4494 * t21398;
    let t21403 = F::new(16.0) / F::new(15.0) * t12314 * t6756;
    let t21406 = F::new(16.0) / F::new(5.0) * t3965 * t6762 * t6492;
    let t21409 = F::new(16.0) / F::new(3.0) * t3965 * t6766 * t6352;
    let t21410 = t21397 * t348;
    let t21413 = F::new(8.0) / F::new(3.0) * t4488 * t12439 * t21410;
    (t21398, t21401, t21403, t21406, t21409, t21410, t21413)
}
