//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 751/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk751(t143: f64, t2210: f64, t2977: f64, t3255: f64, t3257: f64, t3260: f64, t3263: f64, t4044: f64, t7578: f64, t7590: f64, t7602: f64, t7741: f64, t7752: f64, t7755: f64, t7757: f64, t7768: f64) -> f64 {
    let t7771 = 4.937333717448355_f64 * t4044 * t7602 - 4.937333717448355_f64 * t2977 * t2210 + 4.937333717448355_f64 * t7741 + 4.937333717448355_f64 * t4044 * t7590 + 9.87466743489671_f64 * t4044 * t7578 + 19.489173774580152_f64 * t3255 + 19.489173774580152_f64 * t3257 - 19.489173774580152_f64 * t3260 + 1.8805371096875316_f64 * t3263 - 3.159189221415045_f64 * t7752 - 3.159189221415045_f64 * t7755 + 1.4760499452555382_f64 * t7757 + 3.7610742193750633_f64 * t143 * t7768;
    t7771
}
