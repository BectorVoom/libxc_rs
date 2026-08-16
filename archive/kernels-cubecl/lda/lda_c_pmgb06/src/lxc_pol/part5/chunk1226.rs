//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1226/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1226<F: Float>(t12448: F, t12450: F, t12461: F, t12463: F, t19983: F, t19985: F, t19986: F, t19987: F, t19988: F, t19992: F, t19995: F, t19998: F, t20001: F, t20009: F, t20012: F, t20014: F, t20017: F, t20021: F, t20025: F, t20028: F, t20031: F, t20035: F, t20039: F) -> (F, F) {
    let t21948 = -t12448 - t12450 + t19983 - t19985 - t12461 - t12463 + t19986 + t19987 + t19988 + t19992 + t19995;
    let t21949 = -t19998 - t20001 - t20009 - t20012 - t20014 - t20017 - t20021 + t20025 - t20028 + t20031 - t20035 - t20039;
    (t21948, t21949)
}
