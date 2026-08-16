//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1213/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1213<F: Float>(t15582: F, t2002: F, t21885: F, t21888: F, t21889: F, t21890: F, t21894: F, t21897: F, t21900: F, t21903: F, t21905: F, t21906: F, t21907: F, t21909: F) -> (F, F) {
    let t21911 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t15582 * t2002;
    let t21912 = -t21885 + t21888 + t21889 + t21890 - t21894 - t21897 + t21900 - t21903 + t21905 + t21906 + t21907 + t21909 + t21911;
    (t21911, t21912)
}
