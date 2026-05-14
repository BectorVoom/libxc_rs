//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1035/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1035<F: Float>(t11807: F, t11808: F, t11815: F, t11816: F, t11820: F, t11823: F, t11825: F, t11827: F, t11829: F, t11831: F, t11833: F, t11835: F, t11837: F, t11842: F, t11843: F, t11845: F, t11846: F, t11847: F, t11853: F, t11859: F, t11861: F, t11865: F, t11867: F) -> (F, F) {
    let t14322 = t11807 - t11808 + t11815 - t11816 + t11820 + t11823 + t11825 + t11827 + t11829 + t11831 + t11833 - t11835;
    let t14325 = -t11837 - t11842 + t11843 - t11845 + t11846 - t11847 + t11853 + t11859 - t11861 + t11865 - t11867;
    (t14322, t14325)
}
