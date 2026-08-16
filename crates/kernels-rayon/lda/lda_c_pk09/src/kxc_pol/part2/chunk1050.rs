//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1050/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1050(t11101: f64, t537: f64, t11096: f64, t11380: f64, t11386: f64, t11389: f64, t11393: f64, t11396: f64, t11401: f64, t11403: f64, t11407: f64, t11413: f64, t11416: f64, t11420: f64, t11424: f64, t11426: f64, t1805: f64, t1901: f64, t1904: f64, t1944: f64, t2016: f64, t2032: f64, t2783: f64, t2920: f64, t6781: f64, t6829: f64, t6831: f64) -> f64 {
    let t11429 = t537 * t11101;
    let t11432 = 7.35994946043302_f64 * t6829 + 18.635258017632964_f64 * t11380 + 37.27051603526593_f64 * t1901 * t11096 + 0.04115066352984959_f64 * t11386 + 0.04115066352984959_f64 * t1904 * t11389 + 1.2536914064583544_f64 * t6831 + 2.0_f64 * t6781 * t11393 - t11396 * t2016 - 4.937333717448355_f64 * t1944 * t2783 - 4.937333717448355_f64 * t11401 * t11403 - 4.937333717448355_f64 * t11407 - 4.937333717448355_f64 * t2920 * t2032 - 0.04115066352984959_f64 * t11413 - 0.08230132705969918_f64 * t1904 * t11416 + 0.04115066352984959_f64 * t1904 * t11420 - 1.8805371096875316_f64 * t11424 * t11426 + 2.2140749178833072_f64 * t11429 * t1805;
    t11432
}
