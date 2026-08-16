//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 350/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk350(t51: f64, t420: f64, t630: f64, t1207: f64, t1204: f64, t1709: f64, t1713: f64, t425: f64, t1711: f64, t620: f64, t1197: f64, t1193: f64, t427: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t1714 = t420 * t630;
    let t1715 = t1714 * t1207;
    let t1718 = t1709 * t1204 + 1.28_f64 * t1713 * t1715;
    let t1719 = t425 * t1718;
    let t1720 = piecewise3(t52, t1711, t1719);
    let t1722 = t420 * t620;
    let t1723 = t1722 * t1197;
    let t1726 = t1709 * t1193 + 1.28_f64 * t1713 * t1723;
    let t1727 = t427 * t1726;
    (t1719, t1720, t1727)
}
