//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 861/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk861<F: Float>(t10429: F, t1625: F, t10001: F, t10405: F, t10409: F, t10412: F, t10416: F, t10421: F, t10424: F, t10426: F, t1348: F, t1478: F, t1483: F, t1495: F, t2559: F, t297: F, t9973: F, t9975: F, t9980: F, t9983: F, t9987: F, t9989: F, t9995: F, t9998: F) -> (F,) {
    let t10430 = t10429 * t1625;
    let t10432 = -t1495 * t9973 + 0.7380249726277691 * t9975 - 16.20073542583857 * t9980 + 10.80049028389238 * t9983 + 3.7610742193750633 * t9987 - 3.7610742193750633 * t9989 * t1478 + 3.7610742193750633 * t2559 * t1483 - 22.07984838129906 * t9995 - 44.15969676259812 * t9998 + 10.80049028389238 * t10001 + t297 * t10405 - 0.04115066352984959 * t10409 - 0.08230132705969918 * t1348 * t10412 + 0.04115066352984959 * t1348 * t10416 + 0.04115066352984959 * t10421 + 2.427516195194328 * t10424 * t10426 + 2.427516195194328 * t10430;
    (t10432,)
}
