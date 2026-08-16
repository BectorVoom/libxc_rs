//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 661/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk661<F: Float>(t4044: F, t73: F, t1311: F, t1322: F, t1289: F, t107: F, t1126: F, t410: F, t1180: F, t701: F, t2854: F, t2856: F, t2859: F, t2861: F, t2863: F, t2868: F, t2870: F, t2875: F, t2879: F, t2882: F, t2884: F, t2889: F, t2891: F, t2893: F, t2895: F, t2947: F) -> (F, F, F, F, F, F) {
    let t4045 = t73 * t4044;
    let t4049 = t1311 * t1322;
    let t4053 = t73 * t1289;
    let t4060 = t107 * t410 * t1126;
    let t4063 = t107 * t1180 * t701;
    let t4065 = t2854 - t2856 - t2859 - t2861 - t2863 + t2868 - t2870 + t2875 + t2879 - t2882 - t2884 - t2889 - t2891 + t2893 + t2895 + t2947;
    (t4045, t4049, t4053, t4060, t4063, t4065)
}
