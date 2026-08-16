//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1153/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1153(t2466: f64, t5305: f64, t1972: f64, t6541: f64, t6545: f64, t17666: f64, t17668: f64, t13707: f64, t20843: f64, t20845: f64, t20847: f64, t20849: f64, t20852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20854 = t5305 * t2466 / 15.0_f64;
    let t20856 = t1972 * t6541 / 15.0_f64;
    let t20858 = t1972 * t6545 / 15.0_f64;
    let t20859 = 4.0_f64 / 15.0_f64 * t17666;
    let t20860 = 2.0_f64 / 5.0_f64 * t17668;
    let t20861 = -t20843 - t20845 - t20847 - t20849 + t13707 + t20852 + t20854 + t20856 + t20858 + t20859 - t20860;
    (t20854, t20856, t20858, t20859, t20860, t20861)
}
