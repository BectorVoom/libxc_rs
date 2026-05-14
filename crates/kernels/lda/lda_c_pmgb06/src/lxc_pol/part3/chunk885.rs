//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 885/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk885<F: Float>(t3146: F, t853: F, t11813: F, t11815: F, t11816: F, t11820: F, t11823: F, t11825: F, t11827: F, t11829: F, t11831: F, t11833: F, t11835: F, t1499: F, t2101: F, t9317: F) -> (F, F, F, F) {
    let t11837 = t3146 * t853 / 30.0;
    let t11838 = -0.013506172839506173 * t11813 + t11815 - t11816 + t11820 + t11823 + t11825 + t11827 + t11829 + t11831 + t11833 - t11835 - t11837;
    let t11842 = t1499 * t2101 / 10.0;
    let t11843 = 2.0 / 15.0 * t9317;
    (t11837, t11838, t11842, t11843)
}
