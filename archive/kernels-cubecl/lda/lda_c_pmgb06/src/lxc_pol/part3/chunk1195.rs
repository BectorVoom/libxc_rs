//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1195/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1195<F: Float>(t12042: F, t12047: F, t12051: F, t12055: F, t12058: F, t12062: F, t12066: F, t12068: F, t12071: F, t12076: F, t12078: F, t12083: F, t12085: F, t12088: F, t12091: F, t12095: F, t12097: F, t12099: F, t12101: F, t12103: F, t12106: F, t12108: F, t12110: F) -> (F, F) {
    let t14338 = t12042 + t12047 + t12051 - t12055 + t12058 + t12062 + t12066 - t12068 - t12071 - t12076 - t12078;
    let t14339 = -t12083 + t12085 - t12088 + t12091 + t12095 + t12097 + t12099 + t12101 + t12103 + t12106 + t12108 - t12110;
    (t14338, t14339)
}
