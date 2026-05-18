//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1191/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1191<F: Float>(t11783: F, t11784: F, t11785: F, t11786: F, t11790: F, t11793: F, t11795: F, t11802: F, t11804: F, t11805: F, t11806: F, t11807: F, t11808: F, t11815: F, t11816: F, t11820: F, t11823: F, t11825: F, t11827: F, t11829: F, t11831: F, t11833: F, t11835: F) -> (F, F) {
    let t14321 = -t11783 - t11784 + t11785 + t11786 - t11790 - t11793 - t11795 + t11802 - t11804 - t11805 - t11806;
    let t14322 = t11807 - t11808 + t11815 - t11816 + t11820 + t11823 + t11825 + t11827 + t11829 + t11831 + t11833 - t11835;
    (t14321, t14322)
}
