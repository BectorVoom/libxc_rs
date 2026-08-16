//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 741/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk741(t2984: f64, t3032: f64, t3034: f64, t3090: f64, t3102: f64, t3105: f64, t3107: f64, t3120: f64, t4715: f64, t4725: f64, t7661: f64, t7665: f64, t7671: f64, t7674: f64, t7678: f64) -> f64 {
    let t7686 = -0.04115066352984959_f64 * t4715 * t7661 + 0.04115066352984959_f64 * t4715 * t7665 - 0.04115066352984959_f64 * t7671 - 0.04115066352984959_f64 * t4715 * t7674 - 0.08230132705969918_f64 * t4725 * t7678 + 1.6183441301295518_f64 * t2984 + 2.507382812916709_f64 * t3032 + 0.4178971354861182_f64 * t3034 + t3090 + t3102 - 2.400108951976084_f64 * t3105 - 2.400108951976084_f64 * t3107 - t3120;
    t7686
}
