//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 474/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk474(t1614: f64, t2606: f64, t1451: f64, t1611: f64, t1627: f64, t1639: f64, t1644: f64, t1649: f64, t1651: f64, t2559: f64, t2568: f64, t2571: f64, t2580: f64, t2583: f64, t2587: f64, t2596: f64, t307: f64, t311: f64, t319: f64, t328: f64) -> (f64, f64) {
    let t2607 = t2606 * t1614;
    let t2610 = t2559 * t1611 / 12.0_f64 - t2568 * t311 / 6.0_f64 - t2571 * t311 / 6.0_f64 - t2580 * t311 / 6.0_f64 - t2583 * t311 / 6.0_f64 + t319 * t2587 / 6.0_f64 - t2596 * t1451 / 6.0_f64 - t328 * t2587 / 6.0_f64 + t307 * t2587 / 6.0_f64 - t2607 * t1451 / 6.0_f64 + t1627 - t1639 + t1644 - t1649 - t1651;
    (t2607, t2610)
}
