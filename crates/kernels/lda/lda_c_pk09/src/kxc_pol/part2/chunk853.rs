//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 853/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk853<F: Float>(t2596: F, t5569: F, t10116: F, t10240: F, t10262: F, t10270: F, t10275: F, t10281: F, t10287: F, t10295: F, t10297: F, t1451: F, t1565: F, t1568: F, t1581: F, t1594: F, t1629: F, t1634: F, t2552: F, t2555: F, t2559: F, t2583: F, t2587: F, t307: F, t319: F, t5716: F, t5796: F, t5886: F) -> (F,) {
    let t10299 = t2596 * t5569;
    let t10301 = t1568 * t2587 / 6.0 + t307 * t10240 / 6.0 - t10262 * t1451 / 6.0 - t1594 * t2587 / 6.0 - t2559 * t5796 / 6.0 - t10270 / 6.0 - t2555 * t1629 / 6.0 + t5716 * t10275 / 12.0 - t2552 * t1629 / 6.0 - t10281 / 6.0 + t2583 * t1629 / 6.0 - t1634 * t10116 / 6.0 + 0.04991874779241519 * t10287 + t1581 * t2587 / 6.0 + t1565 * t2587 / 6.0 + t319 * t10240 / 6.0 + t10295 / 18.0 - t10297 / 18.0 + t10299 / 18.0 - t5886;
    (t10301,)
}
