//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 977/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk977<F: Float>(t10423: F, t306: F, t1380: F, t309: F, t310: F, t1336: F, t2689: F, t1625: F, t10001: F, t10405: F, t10409: F, t10412: F, t10416: F, t10421: F, t1348: F, t1478: F, t1483: F, t1495: F, t2559: F, t297: F, t9973: F, t9975: F, t9980: F, t9983: F, t9987: F, t9989: F, t9995: F, t9998: F) -> F {
    let t10424 = t10423 * t306;
    let t10426 = t309 * t310 * t1380;
    let t10429 = t2689 * t1336;
    let t10430 = t10429 * t1625;
    let t10432 = -t1495 * t9973 + F::cast_from(0.7380249726277691_f64) * t9975 - F::cast_from(16.20073542583857_f64) * t9980 + F::cast_from(10.80049028389238_f64) * t9983 + F::cast_from(3.7610742193750633_f64) * t9987 - F::cast_from(3.7610742193750633_f64) * t9989 * t1478 + F::cast_from(3.7610742193750633_f64) * t2559 * t1483 - F::cast_from(22.07984838129906_f64) * t9995 - F::cast_from(44.15969676259812_f64) * t9998 + F::cast_from(10.80049028389238_f64) * t10001 + t297 * t10405 - F::cast_from(0.04115066352984959_f64) * t10409 - F::cast_from(0.08230132705969918_f64) * t1348 * t10412 + F::cast_from(0.04115066352984959_f64) * t1348 * t10416 + F::cast_from(0.04115066352984959_f64) * t10421 + F::cast_from(2.427516195194328_f64) * t10424 * t10426 + F::cast_from(2.427516195194328_f64) * t10430;
    t10432
}
