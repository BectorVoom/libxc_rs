//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1050/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1050<F: Float>(t11101: F, t537: F, t11096: F, t11380: F, t11386: F, t11389: F, t11393: F, t11396: F, t11401: F, t11403: F, t11407: F, t11413: F, t11416: F, t11420: F, t11424: F, t11426: F, t1805: F, t1901: F, t1904: F, t1944: F, t2016: F, t2032: F, t2783: F, t2920: F, t6781: F, t6829: F, t6831: F) -> F {
    let t11429 = t537 * t11101;
    let t11432 = F::cast_from(7.35994946043302_f64) * t6829 + F::cast_from(18.635258017632964_f64) * t11380 + F::cast_from(37.27051603526593_f64) * t1901 * t11096 + F::cast_from(0.04115066352984959_f64) * t11386 + F::cast_from(0.04115066352984959_f64) * t1904 * t11389 + F::cast_from(1.2536914064583544_f64) * t6831 + F::cast_from(2.0_f64) * t6781 * t11393 - t11396 * t2016 - F::cast_from(4.937333717448355_f64) * t1944 * t2783 - F::cast_from(4.937333717448355_f64) * t11401 * t11403 - F::cast_from(4.937333717448355_f64) * t11407 - F::cast_from(4.937333717448355_f64) * t2920 * t2032 - F::cast_from(0.04115066352984959_f64) * t11413 - F::cast_from(0.08230132705969918_f64) * t1904 * t11416 + F::cast_from(0.04115066352984959_f64) * t1904 * t11420 - F::cast_from(1.8805371096875316_f64) * t11424 * t11426 + F::cast_from(2.2140749178833072_f64) * t11429 * t1805;
    t11432
}
