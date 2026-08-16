//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1016/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1016(t11019: f64, t6413: f64, t2: f64, t420: f64, t9727: f64, t2715: f64, t9731: f64, t11004: f64, t11013: f64, t11016: f64, t1204: f64, t1207: f64, t1713: f64, t2711: f64, t2716: f64, t630: f64, t6409: f64) -> (f64, f64) {
    let t11020 = t11019 * t6413;
    let t11023 = t420 * t2;
    let t11024 = t11023 * t9727;
    let t11027 = t2715 * t9731;
    let t11030 = t11004 * t1204 + t2711 * t630 * t1207 + t11013 - t11016 + 1.28_f64 * t6409 * t2716 - 1.28_f64 * t1713 * t11020 - 2.56_f64 * t1713 * t11024 - 1.28_f64 * t1713 * t11027;
    (t11023, t11030)
}
