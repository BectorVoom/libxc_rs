//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 747/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk747(t2972: f64, t7731: f64, t2341: f64, t3195: f64, t3201: f64, t3203: f64, t3224: f64, t3231: f64, t3234: f64, t3239: f64, t3243: f64, t3246: f64, t4044: f64, t709: f64, t7598: f64, t7727: f64, t806: f64) -> f64 {
    let t7732 = t2972 * t7731;
    let t7736 = -22.07984838129906_f64 * t3195 - 22.07984838129906_f64 * t3201 + 1.4760499452555382_f64 * t3203 + 2.2140749178833072_f64 * t3224 + 12.423505345088643_f64 * t3231 + 12.423505345088643_f64 * t3234 + t3239 - 3.159189221415045_f64 * t3243 + 1.8805371096875316_f64 * t806 * t2341 + 0.7897973053537612_f64 * t3246 + 2.2140749178833072_f64 * t7727 * t709 + 4.937333717448355_f64 * t7732 + 9.87466743489671_f64 * t4044 * t7598;
    t7736
}
