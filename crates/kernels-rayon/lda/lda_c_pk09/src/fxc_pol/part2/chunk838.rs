//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 838/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk838(t119: f64, t2418: f64, t3254: f64, t7731: f64, t155: f64, t7991: f64, t151: f64, t8141: f64, t1062: f64, t2238: f64, t721: f64, t1067: f64, t2271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8555 = t2418 * t119;
    let t8560 = t3254 * t7731;
    let t8564 = t155 * t7991;
    let t8566 = t151 * t8141;
    let t8570 = t2238 * t1062;
    let t8571 = t8570 * t721;
    let t8573 = t2271 * t1067;
    (t8555, t8560, t8564, t8566, t8571, t8573)
}
