//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 225/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk225(t143: f64, t573: f64, t168: f64, t650: f64, t96: f64, t151: f64, t161: f64, t164: f64, t709: f64, t713: f64, t756: f64, t790: f64, t806: f64, t812: f64, t827: f64, t833: f64, t843: f64, t846: f64, t851: f64, t864: f64, t870: f64, t98: f64) -> (f64, f64, f64, f64) {
    let t872 = 2.507382812916709_f64 * t143 * t573;
    let t873 = t650 * t168;
    let t874 = t96 * t873;
    let t879 = -2.427516195194328_f64 * t790 * t98 + 1.8805371096875316_f64 * t806 * t98 - t812 - 0.04115066352984959_f64 * t164 * t827 - t833 - 1.8805371096875316_f64 * t151 * t713 - 1.8805371096875316_f64 * t151 * t709 + t843 + 0.04115066352984959_f64 * t164 * t846 + 0.04115066352984959_f64 * t164 * t851 - 4.937333717448355_f64 * t161 * t709 + 4.937333717448355_f64 * t864 * t98 - 4.937333717448355_f64 * t161 * t713 - t870 + t872 - 0.04115066352984959_f64 * t164 * t874 + 4.937333717448355_f64 * t161 * t756;
    (t872, t873, t874, t879)
}
