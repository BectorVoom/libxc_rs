//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 807/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk807(t1091: f64, t161: f64, t2341: f64, t3568: f64, t3578: f64, t3580: f64, t3613: f64, t3614: f64, t3616: f64, t3662: f64, t3665: f64, t8089: f64, t8093: f64, t8096: f64, t8101: f64, t864: f64) -> f64 {
    let t8112 = -t8089 * t1091 - 4.937333717448355_f64 * t8093 - 4.937333717448355_f64 * t161 * t8096 - 4.937333717448355_f64 * t161 * t8101 + 4.937333717448355_f64 * t864 * t2341 + 0.027433775686566395_f64 * t3568 + 0.04115066352984959_f64 * t3578 - 3.2915558116322368_f64 * t3580 + t3613 + 3.2915558116322368_f64 * t3614 + 3.2915558116322368_f64 * t3616 - 18.635258017632964_f64 * t3662 - t3665;
    t8112
}
