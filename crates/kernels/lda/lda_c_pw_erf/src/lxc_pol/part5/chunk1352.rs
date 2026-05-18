//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1352/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1352<F: Float>(t22872: F, t22875: F, t22880: F, t22885: F, t22889: F, t22890: F, t22892: F, t22894: F, t22898: F, t22900: F, t22902: F, t22904: F, t22906: F) -> F {
    let t23324 = -t22872 + t22875 - t22880 - t22885 + t22889 - t22890 + t22892 + t22894 + t22898 - t22900 + t22902 - t22904 - t22906;
    t23324
}
