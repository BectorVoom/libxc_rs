//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1191/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1191<F: Float>(t4794: F, t571: F, t7418: F, t1318: F, t7719: F, t3859: F, t519: F, t7651: F, t3802: F, t7691: F, t1325: F, t7687: F) -> (F, F, F, F, F) {
    let t21604 = t571 * t4794 * t7418;
    let t21605 = F::new(8.0) / F::new(27.0) * t21604;
    let t21607 = t1318 * t4794 * t7719;
    let t21608 = F::new(16.0) / F::new(27.0) * t21607;
    let t21610 = t519 * t3859 * t7651;
    let t21611 = F::new(16.0) / F::new(45.0) * t21610;
    let t21613 = t519 * t3802 * t7691;
    let t21614 = F::new(8.0) / F::new(45.0) * t21613;
    let t21616 = t1325 * t3859 * t7687;
    (t21605, t21608, t21611, t21614, t21616)
}
