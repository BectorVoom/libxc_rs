//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 747/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk747<F: Float>(t2972: F, t7731: F, t2341: F, t3195: F, t3201: F, t3203: F, t3224: F, t3231: F, t3234: F, t3239: F, t3243: F, t3246: F, t4044: F, t709: F, t7598: F, t7727: F, t806: F) -> F {
    let t7732 = t2972 * t7731;
    let t7736 = -F::cast_from(22.07984838129906_f64) * t3195 - F::cast_from(22.07984838129906_f64) * t3201 + F::cast_from(1.4760499452555382_f64) * t3203 + F::cast_from(2.2140749178833072_f64) * t3224 + F::cast_from(12.423505345088643_f64) * t3231 + F::cast_from(12.423505345088643_f64) * t3234 + t3239 - F::cast_from(3.159189221415045_f64) * t3243 + F::cast_from(1.8805371096875316_f64) * t806 * t2341 + F::cast_from(0.7897973053537612_f64) * t3246 + F::cast_from(2.2140749178833072_f64) * t7727 * t709 + F::cast_from(4.937333717448355_f64) * t7732 + F::cast_from(9.87466743489671_f64) * t4044 * t7598;
    t7736
}
