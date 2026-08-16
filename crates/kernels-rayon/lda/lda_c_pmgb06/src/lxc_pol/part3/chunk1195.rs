//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1195/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1195(t12042: f64, t12047: f64, t12051: f64, t12055: f64, t12058: f64, t12062: f64, t12066: f64, t12068: f64, t12071: f64, t12076: f64, t12078: f64, t12083: f64, t12085: f64, t12088: f64, t12091: f64, t12095: f64, t12097: f64, t12099: f64, t12101: f64, t12103: f64, t12106: f64, t12108: f64, t12110: f64) -> (f64, f64) {
    let t14338 = t12042 + t12047 + t12051 - t12055 + t12058 + t12062 + t12066 - t12068 - t12071 - t12076 - t12078;
    let t14339 = -t12083 + t12085 - t12088 + t12091 + t12095 + t12097 + t12099 + t12101 + t12103 + t12106 + t12108 - t12110;
    (t14338, t14339)
}
