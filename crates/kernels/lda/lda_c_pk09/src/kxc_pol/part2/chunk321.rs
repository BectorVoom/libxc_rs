//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 321/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk321<F: Float>(t1214: F, t365: F, t355: F, t1243: F, t1255: F, t1263: F, t1272: F, t1251: F, t1259: F, t1268: F, t1275: F, t300: F, t306: F, t1215: F, t318: F, t304: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1546 = t365 * t1214;
    let t1549 = t355 * t1214;
    let t1552 = 1.4770435158815312 * t1243;
    let t1554 = 0.49234783862717707 * t1255;
    let t1556 = 0.2946275542389858 * t1263;
    let t1558 = 0.0982091847463286 * t1272;
    let t1560 = t1552 - 1.4770435158815312 * t1251 + t1554 + 1.4770435158815312 * t1259 + t1556 - 0.2946275542389858 * t1268 + t1558 + 0.2946275542389858 * t1275;
    let t1561 = t300 * t1560;
    let t1562 = t1561 * t306;
    let t1565 = t318 * t1215;
    let t1568 = t304 * t1215;
    (t1546, t1549, t1552, t1554, t1556, t1558, t1560, t1561, t1562, t1565, t1568)
}
