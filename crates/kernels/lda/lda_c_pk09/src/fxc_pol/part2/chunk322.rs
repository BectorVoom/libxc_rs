//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 322/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk322<F: Float>(t1243: F, t1255: F, t1263: F, t1272: F, t1251: F, t1259: F, t1268: F, t1275: F, t314: F, t306: F, t323: F, t1215: F, t327: F, t1220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1571 = 0.7661514025603425 * t1243;
    let t1573 = 0.2553838008534475 * t1255;
    let t1575 = 0.15282509383508946 * t1263;
    let t1577 = 0.05094169794502982 * t1272;
    let t1579 = t1571 - 0.7661514025603425 * t1251 + t1573 + 0.7661514025603425 * t1259 + t1575 - 0.15282509383508946 * t1268 + t1577 + 0.15282509383508946 * t1275;
    let t1580 = t314 * t1579;
    let t1581 = t1580 * t306;
    let t1584 = 1.5323028051206833 * t1243;
    let t1586 = 0.5107676017068944 * t1255;
    let t1588 = 0.3056501876701794 * t1263;
    let t1590 = 0.1018833958900598 * t1272;
    let t1592 = t1584 - 1.5323028051206833 * t1251 + t1586 + 1.5323028051206833 * t1259 + t1588 - 0.3056501876701794 * t1268 + t1590 + 0.3056501876701794 * t1275;
    let t1593 = t323 * t1592;
    let t1594 = t1593 * t306;
    let t1597 = t327 * t1215;
    let t1601 = 0.01233429741534199 * t1220;
    (t1571, t1573, t1575, t1577, t1579, t1580, t1581, t1584, t1586, t1588, t1590, t1592, t1593, t1594, t1597, t1601)
}
