//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 726/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk726<F: Float>(t1091: F, t161: F, t2341: F, t3568: F, t3578: F, t3580: F, t3613: F, t3614: F, t3616: F, t3662: F, t3665: F, t8089: F, t8093: F, t8096: F, t8101: F, t864: F) -> (F,) {
    let t8112 = -t8089 * t1091 - 4.937333717448355 * t8093 - 4.937333717448355 * t161 * t8096 - 4.937333717448355 * t161 * t8101 + 4.937333717448355 * t864 * t2341 + 0.027433775686566395 * t3568 + 0.04115066352984959 * t3578 - 3.2915558116322368 * t3580 + t3613 + 3.2915558116322368 * t3614 + 3.2915558116322368 * t3616 - 18.635258017632964 * t3662 - t3665;
    (t8112,)
}
