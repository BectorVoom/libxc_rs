//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 371/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk371(t1684: f64, t1735: f64, t1732: f64, t1738: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1811 = 11.879313099038017_f64 * t1684;
    let t1813 = 3.959771033012672_f64 * t1735;
    let t1815 = t1811 - 11.879313099038017_f64 * t1732 + t1813 + 11.879313099038017_f64 * t1738;
    let t1816 = t446 * t446;
    let t1817 = t1816 + 1.0_f64;
    let t1818 = 1.0_f64 / t1817;
    let t1819 = t1815 * t1818;
    (t1811, t1813, t1815, t1817, t1818, t1819)
}
