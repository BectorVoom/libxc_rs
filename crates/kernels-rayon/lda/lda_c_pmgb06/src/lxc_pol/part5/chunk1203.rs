//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1203/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1203(t11090: f64, t11093: f64, t11095: f64, t11098: f64, t11100: f64, t11101: f64, t14984: f64, t8647: f64, t8651: f64, t8655: f64, t8659: f64, t8668: f64, t8684: f64, t8685: f64, t8692: f64, t8693: f64, t8723: f64) -> f64 {
    let t21781 = -t8647 - t8651 + t8655 + t8659 + t8668 + 3.0_f64 * t14984 + 180.0_f64 * t11090 + t11093 + 72.0_f64 * t11095 + t11098 - t11100 - 360.0_f64 * t11101 - t8684 - 1025.4018858216407_f64 * t8685 + t8692 - 0.5848223622634646_f64 * t8693 - t8723;
    t21781
}
