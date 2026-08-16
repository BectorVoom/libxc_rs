//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1278/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1278(t2171: f64, t6332: f64, t6337: f64, t4738: f64, t6344: f64, t22880: f64, t22885: f64, t22889: f64, t22890: f64, t22892: f64, t22894: f64, t22898: f64, t22900: f64, t22902: f64, t22904: f64) -> (f64, f64, f64, f64) {
    let t22906 = 8.0_f64 / 15.0_f64 * t2171 * t6332;
    let t22908 = 4.0_f64 / 9.0_f64 * t2171 * t6337;
    let t22910 = 8.0_f64 / 9.0_f64 * t4738 * t6344;
    let t22911 = -t22880 - t22885 + t22889 - t22890 + t22892 + t22894 + t22898 - t22900 + t22902 - t22904 - t22906 + t22908 + t22910;
    (t22906, t22908, t22910, t22911)
}
