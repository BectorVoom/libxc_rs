//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 977/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk977(t10423: f64, t306: f64, t1380: f64, t309: f64, t310: f64, t1336: f64, t2689: f64, t1625: f64, t10001: f64, t10405: f64, t10409: f64, t10412: f64, t10416: f64, t10421: f64, t1348: f64, t1478: f64, t1483: f64, t1495: f64, t2559: f64, t297: f64, t9973: f64, t9975: f64, t9980: f64, t9983: f64, t9987: f64, t9989: f64, t9995: f64, t9998: f64) -> f64 {
    let t10424 = t10423 * t306;
    let t10426 = t309 * t310 * t1380;
    let t10429 = t2689 * t1336;
    let t10430 = t10429 * t1625;
    let t10432 = -t1495 * t9973 + 0.7380249726277691_f64 * t9975 - 16.20073542583857_f64 * t9980 + 10.80049028389238_f64 * t9983 + 3.7610742193750633_f64 * t9987 - 3.7610742193750633_f64 * t9989 * t1478 + 3.7610742193750633_f64 * t2559 * t1483 - 22.07984838129906_f64 * t9995 - 44.15969676259812_f64 * t9998 + 10.80049028389238_f64 * t10001 + t297 * t10405 - 0.04115066352984959_f64 * t10409 - 0.08230132705969918_f64 * t1348 * t10412 + 0.04115066352984959_f64 * t1348 * t10416 + 0.04115066352984959_f64 * t10421 + 2.427516195194328_f64 * t10424 * t10426 + 2.427516195194328_f64 * t10430;
    t10432
}
