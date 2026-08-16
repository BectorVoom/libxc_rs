//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 968/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk968(t2607: f64, t5569: f64, t10059: f64, t307: f64, t2596: f64, t10116: f64, t10240: f64, t10262: f64, t10270: f64, t10275: f64, t10281: f64, t10287: f64, t1451: f64, t1565: f64, t1568: f64, t1581: f64, t1594: f64, t1629: f64, t1634: f64, t2552: f64, t2555: f64, t2559: f64, t2583: f64, t2587: f64, t319: f64, t5716: f64, t5796: f64, t5886: f64) -> f64 {
    let t10295 = t2607 * t5569;
    let t10297 = t307 * t10059;
    let t10299 = t2596 * t5569;
    let t10301 = t1568 * t2587 / 6.0_f64 + t307 * t10240 / 6.0_f64 - t10262 * t1451 / 6.0_f64 - t1594 * t2587 / 6.0_f64 - t2559 * t5796 / 6.0_f64 - t10270 / 6.0_f64 - t2555 * t1629 / 6.0_f64 + t5716 * t10275 / 12.0_f64 - t2552 * t1629 / 6.0_f64 - t10281 / 6.0_f64 + t2583 * t1629 / 6.0_f64 - t1634 * t10116 / 6.0_f64 + 0.04991874779241519_f64 * t10287 + t1581 * t2587 / 6.0_f64 + t1565 * t2587 / 6.0_f64 + t319 * t10240 / 6.0_f64 + t10295 / 18.0_f64 - t10297 / 18.0_f64 + t10299 / 18.0_f64 - t5886;
    t10301
}
