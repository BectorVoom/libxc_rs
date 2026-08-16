//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 661/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk661(t4044: f64, t73: f64, t1311: f64, t1322: f64, t1289: f64, t107: f64, t1126: f64, t410: f64, t1180: f64, t701: f64, t2854: f64, t2856: f64, t2859: f64, t2861: f64, t2863: f64, t2868: f64, t2870: f64, t2875: f64, t2879: f64, t2882: f64, t2884: f64, t2889: f64, t2891: f64, t2893: f64, t2895: f64, t2947: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4045 = t73 * t4044;
    let t4049 = t1311 * t1322;
    let t4053 = t73 * t1289;
    let t4060 = t107 * t410 * t1126;
    let t4063 = t107 * t1180 * t701;
    let t4065 = t2854 - t2856 - t2859 - t2861 - t2863 + t2868 - t2870 + t2875 + t2879 - t2882 - t2884 - t2889 - t2891 + t2893 + t2895 + t2947;
    (t4045, t4049, t4053, t4060, t4063, t4065)
}
