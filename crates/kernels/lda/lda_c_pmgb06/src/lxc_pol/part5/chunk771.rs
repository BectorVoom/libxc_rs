//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 771/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk771<F: Float>(t3392: F, t3395: F, t6733: F, t6738: F, t6740: F, t6743: F, t6746: F, t6750: F, t6754: F, t6758: F, t6763: F, t6768: F, t6772: F, t6777: F, t6779: F) -> F {
    let t7224 = F::new(8.0) / F::new(3.0) * t3392 + t3395 - t6733 - t6738 - t6740 - t6743 - t6746 - t6750 + t6754 - t6758 - t6763 + t6768 - t6772 - t6777 - t6779;
    t7224
}
