//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 741/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk741<F: Float>(t2984: F, t3032: F, t3034: F, t3090: F, t3102: F, t3105: F, t3107: F, t3120: F, t4715: F, t4725: F, t7661: F, t7665: F, t7671: F, t7674: F, t7678: F) -> F {
    let t7686 = -F::cast_from(0.04115066352984959_f64) * t4715 * t7661 + F::cast_from(0.04115066352984959_f64) * t4715 * t7665 - F::cast_from(0.04115066352984959_f64) * t7671 - F::cast_from(0.04115066352984959_f64) * t4715 * t7674 - F::cast_from(0.08230132705969918_f64) * t4725 * t7678 + F::cast_from(1.6183441301295518_f64) * t2984 + F::cast_from(2.507382812916709_f64) * t3032 + F::cast_from(0.4178971354861182_f64) * t3034 + t3090 + t3102 - F::cast_from(2.400108951976084_f64) * t3105 - F::cast_from(2.400108951976084_f64) * t3107 - t3120;
    t7686
}
