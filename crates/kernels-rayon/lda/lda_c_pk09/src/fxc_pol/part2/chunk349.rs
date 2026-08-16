//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 349/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk349(t1702: f64, t1705: f64, t1689: f64, t1692: f64, t1697: f64, t253: f64, t424: f64, t1191: f64, t418: f64) -> (f64, f64, f64, f64, f64) {
    let t1706 = t1702 * t1705;
    let t1709 = t1689 - t1692 + 1.28_f64 * t253 * t1697 - 1.28_f64 * t253 * t1706;
    let t1710 = t424 * t1709;
    let t1711 = t1710 * t1191;
    let t1713 = t253 * t418;
    (t1706, t1709, t1710, t1711, t1713)
}
