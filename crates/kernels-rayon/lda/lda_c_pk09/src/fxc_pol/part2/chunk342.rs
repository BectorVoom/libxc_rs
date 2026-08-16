//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 342/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk342(t51: f64, t1335: f64, t1454: f64, t1516: f64, t1655: f64, t213: f64, t630: f64, t555: f64, t1222: f64, t95: f64, t476: f64, t132: f64, t747: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t1657 = t1335 + t1454 + t1516 + t1655;
    let t1658 = t213 * t1657;
    let t1662 = piecewise3(t52, 0.0_f64, 2.0_f64 * t51 * t630);
    let t1663 = t1662 * t555;
    let t1665 = t1222 * t95;
    let t1666 = t476 * t1665;
    let t1667 = 7.35994946043302_f64 * t1666;
    let t1668 = t747 * t132;
    (t1657, t1658, t1662, t1663, t1665, t1666, t1667, t1668)
}
