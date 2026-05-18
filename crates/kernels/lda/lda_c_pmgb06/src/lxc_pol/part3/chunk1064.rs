//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1064/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1064<F: Float>(t1560: F, t5187: F, t1447: F, t4757: F, t12620: F, t12622: F, t12624: F, t12626: F, t12628: F, t12630: F, t12632: F, t12636: F, t12641: F) -> (F, F, F) {
    let t12643 = F::new(2.0) / F::new(15.0) * t5187 * t1560;
    let t12644 = t1447 * t4757;
    let t12645 = F::new(4.0) / F::new(45.0) * t12644;
    let t12646 = -t12620 + t12622 - t12624 - t12626 - t12628 - t12630 - t12632 + t12636 - t12641 - t12643 - t12645;
    (t12643, t12645, t12646)
}
