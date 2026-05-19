//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 807/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk807<F: Float>(t1091: F, t161: F, t2341: F, t3568: F, t3578: F, t3580: F, t3613: F, t3614: F, t3616: F, t3662: F, t3665: F, t8089: F, t8093: F, t8096: F, t8101: F, t864: F) -> F {
    let t8112 = -t8089 * t1091 - F::cast_from(4.937333717448355_f64) * t8093 - F::cast_from(4.937333717448355_f64) * t161 * t8096 - F::cast_from(4.937333717448355_f64) * t161 * t8101 + F::cast_from(4.937333717448355_f64) * t864 * t2341 + F::cast_from(0.027433775686566395_f64) * t3568 + F::cast_from(0.04115066352984959_f64) * t3578 - F::cast_from(3.2915558116322368_f64) * t3580 + t3613 + F::cast_from(3.2915558116322368_f64) * t3614 + F::cast_from(3.2915558116322368_f64) * t3616 - F::cast_from(18.635258017632964_f64) * t3662 - t3665;
    t8112
}
