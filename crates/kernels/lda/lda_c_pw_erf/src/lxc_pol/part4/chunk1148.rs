//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1148/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1148<F: Float>(t1513: F, t2499: F, t12409: F, t12411: F, t12460: F, t12462: F, t12464: F, t581: F, t6843: F, t1318: F, t1466: F, t549: F, t1294: F, t2425: F, t16882: F, t16886: F, t16889: F, t16890: F, t16891: F, t16893: F, t16895: F, t16897: F, t16899: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16901 = 4.0 / 15.0 * t1513 * t2499;
    let t16902 = 64.0 / 135.0 * t12409;
    let t16903 = 64.0 / 135.0 * t12411;
    let t16904 = 64.0 / 135.0 * t12460;
    let t16905 = 128.0 / 135.0 * t12462;
    let t16906 = 64.0 / 81.0 * t12464;
    let t16907 = t581 * t6843;
    let t16911 = 8.0 / 15.0 * t1318 * t1466 * t16907 * t549;
    let t16912 = t2425 * t1294;
    let t16913 = 8.0 / 45.0 * t16912;
    let t16914 = t16882 + t16886 + t16889 + t16890 + t16891 + t16893 + t16895 + t16897 + t16899 - t16901 - t16902 + t16903 - t16904 - t16905 + t16906 - t16911 - t16913;
    (t16901, t16902, t16903, t16904, t16905, t16906, t16911, t16913, t16914)
}
