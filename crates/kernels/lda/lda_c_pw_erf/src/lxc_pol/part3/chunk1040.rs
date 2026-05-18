//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1040/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1040<F: Float>(t1446: F, t4859: F, t4862: F, t11701: F, t1326: F, t519: F, t11705: F, t4829: F, t2031: F, t3709: F, t5244: F, t4850: F) -> (F, F, F, F, F, F, F) {
    let t12176 = F::new(8.0) / F::new(3.0) * t1446 * t4859;
    let t12178 = F::new(32.0) / F::new(15.0) * t1446 * t4862;
    let t12181 = F::new(8.0) / F::new(45.0) * t519 * t1326 * t11701;
    let t12184 = F::new(16.0) / F::new(15.0) * t519 * t4829 * t11705;
    let t12186 = F::new(4.0) / F::new(15.0) * t3709 * t2031;
    let t12188 = F::new(4.0) / F::new(15.0) * t1446 * t5244;
    let t12190 = F::new(16.0) / F::new(15.0) * t1446 * t4850;
    (t12176, t12178, t12181, t12184, t12186, t12188, t12190)
}
