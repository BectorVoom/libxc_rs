//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 852/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk852<F: Float>(t11677: F, t2089: F, t933: F, t1973: F, t925: F, t1968: F, t2092: F, t2061: F, t803: F, t1953: F, t790: F, t10967: F, t21: F, t2095: F, t1977: F, t8930: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11678 = 8.0 / 45.0 * t11677;
    let t11695 = t933 * t2089;
    let t11709 = t925 * t1973;
    let t11753 = t925 * t1968;
    let t11754 = 0.03199259259259259 * t11753;
    let t11781 = t933 * t2092;
    let t11829 = t2061 * t803;
    let t11834 = t1953 * t790;
    let t11845 = t21 * t10967;
    let t11846 = t11845 * t2095;
    let t11848 = t8930 * t1977;
    (t11678, t11695, t11709, t11753, t11754, t11781, t11829, t11834, t11845, t11846, t11848)
}
