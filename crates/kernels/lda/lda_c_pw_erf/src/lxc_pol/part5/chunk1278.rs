//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1278/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1278<F: Float>(t2171: F, t6332: F, t6337: F, t4738: F, t6344: F, t22880: F, t22885: F, t22889: F, t22890: F, t22892: F, t22894: F, t22898: F, t22900: F, t22902: F, t22904: F) -> (F, F, F, F) {
    let t22906 = F::new(8.0) / F::new(15.0) * t2171 * t6332;
    let t22908 = F::new(4.0) / F::new(9.0) * t2171 * t6337;
    let t22910 = F::new(8.0) / F::new(9.0) * t4738 * t6344;
    let t22911 = -t22880 - t22885 + t22889 - t22890 + t22892 + t22894 + t22898 - t22900 + t22902 - t22904 - t22906 + t22908 + t22910;
    (t22906, t22908, t22910, t22911)
}
