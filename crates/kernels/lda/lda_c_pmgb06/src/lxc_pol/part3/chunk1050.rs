//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1050/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1050<F: Float>(t12920: F, t12968: F, t12969: F, t12970: F, t12973: F, t12974: F, t12975: F, t12976: F, t12977: F, t12978: F, t12979: F, t12983: F, t12986: F, t12990: F, t12994: F, t12998: F, t13005: F, t13009: F, t13012: F, t13015: F, t13018: F, t13024: F, t13030: F) -> (F, F) {
    let t14402 = t12920 - t12968 - t12969 - t12970 + t12973 + t12974 + t12975 + t12976 + t12977 + t12978 + t12979;
    let t14403 = -t12983 + t12986 + t12990 + t12994 - t12998 + t13005 + t13009 + t13012 - t13015 + t13018 - t13024 + t13030;
    (t14402, t14403)
}
