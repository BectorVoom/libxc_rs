//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1302/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1302<F: Float>(t15015: F, t16877: F, t16882: F, t16886: F, t16889: F, t16890: F, t16891: F, t16893: F, t16895: F, t16897: F, t16899: F, t16901: F, t16902: F, t16903: F, t16904: F, t16905: F, t16906: F) -> (F,) {
    let t19198 = -t16877 + t16882 + t16886 + t16889 + t16890 + t16891 + t16893 + t16895 + t16897 + t16899 - t16901 - t16902 + t16903 - t16904 - t16905 + t16906 + 0.003030876351851852 * t15015;
    (t19198,)
}
