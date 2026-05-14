//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1038/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1038<F: Float>(t12011: F, t12015: F, t12017: F, t12019: F, t12021: F, t12023: F, t12026: F, t12028: F, t12032: F, t12035: F, t12038: F, t12040: F, t12042: F, t12047: F, t12051: F, t12055: F, t12058: F, t12062: F, t12066: F, t12068: F, t12071: F, t12076: F, t12078: F) -> (F, F) {
    let t14336 = -t12011 - t12015 - t12017 - t12019 - t12021 + t12023 + t12026 + t12028 + t12032 + t12035 - t12038 - t12040;
    let t14338 = t12042 + t12047 + t12051 - t12055 + t12058 + t12062 + t12066 - t12068 - t12071 - t12076 - t12078;
    (t14336, t14338)
}
