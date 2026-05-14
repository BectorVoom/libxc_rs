//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1085/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1085<F: Float>(t20987: F, t20992: F, t20995: F, t20999: F, t21001: F, t21003: F, t21005: F, t21007: F, t21009: F, t21013: F, t21016: F, t21021: F, t21026: F, t21028: F, t21033: F, t21036: F, t21038: F, t21041: F, t21044: F, t21050: F, t21052: F, t21055: F, t21059: F, t21065: F) -> (F, F) {
    let t22040 = -t20987 - t20992 - t20995 - t20999 - t21001 + t21003 - t21005 + t21007 + t21009 + t21013 - t21016 - t21021;
    let t22043 = -t21026 - t21028 - t21033 - t21036 + t21038 + t21041 + t21044 + t21050 - t21052 - t21055 - t21059 + t21065;
    (t22040, t22043)
}
