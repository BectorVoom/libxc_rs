//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 333/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk333(t1560: f64, t300: f64, t306: f64, t1215: f64, t318: f64, t304: f64, t1243: f64, t1255: f64, t1263: f64, t1272: f64, t1251: f64, t1259: f64, t1268: f64, t1275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1561 = t300 * t1560;
    let t1562 = t1561 * t306;
    let t1565 = t318 * t1215;
    let t1568 = t304 * t1215;
    let t1571 = 0.7661514025603425_f64 * t1243;
    let t1573 = 0.2553838008534475_f64 * t1255;
    let t1575 = 0.15282509383508946_f64 * t1263;
    let t1577 = 0.05094169794502982_f64 * t1272;
    let t1579 = t1571 - 0.7661514025603425_f64 * t1251 + t1573 + 0.7661514025603425_f64 * t1259 + t1575 - 0.15282509383508946_f64 * t1268 + t1577 + 0.15282509383508946_f64 * t1275;
    (t1561, t1562, t1565, t1568, t1571, t1573, t1575, t1577, t1579)
}
