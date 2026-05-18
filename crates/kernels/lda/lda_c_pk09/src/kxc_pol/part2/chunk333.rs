//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 333/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk333<F: Float>(t1560: F, t300: F, t306: F, t1215: F, t318: F, t304: F, t1243: F, t1255: F, t1263: F, t1272: F, t1251: F, t1259: F, t1268: F, t1275: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1561 = t300 * t1560;
    let t1562 = t1561 * t306;
    let t1565 = t318 * t1215;
    let t1568 = t304 * t1215;
    let t1571 = F::new(0.7661514025603425) * t1243;
    let t1573 = F::new(0.2553838008534475) * t1255;
    let t1575 = F::new(0.15282509383508946) * t1263;
    let t1577 = F::new(0.05094169794502982) * t1272;
    let t1579 = t1571 - F::new(0.7661514025603425) * t1251 + t1573 + F::new(0.7661514025603425) * t1259 + t1575 - F::new(0.15282509383508946) * t1268 + t1577 + F::new(0.15282509383508946) * t1275;
    (t1561, t1562, t1565, t1568, t1571, t1573, t1575, t1577, t1579)
}
